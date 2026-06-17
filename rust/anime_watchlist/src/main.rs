use actix_web::{get, web, App, HttpServer, HttpResponse};
use actix_files::Files;
use sqlx::{Sqlite, SqlitePool};
use serde::{Deserialize, Serialize};
use dotenvy;
use reqwest;

#[derive(Deserialize, Serialize)]
struct JikanAnime {
    mal_id: u32,
    title: String,
    title_english: Option<String>,
    images: JikanImages,
    episodes: Option<u32>,
    synopsis: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct JikanImages {
    jpg: JikanJpg,
}

#[derive(Deserialize, Serialize)]
struct JikanJpg {
    image_url: String,
}

#[derive(Deserialize, Serialize)]
struct JikanResponse {
    data: Vec<JikanAnime>,
}

#[derive(Serialize, Deserialize)]
struct WatchlistItem {
    id: i64,
    mal_id: i64,
    title: String,
    image_url: Option<String>,
    status: String,
    episodes_watched: i64,
    rating: Option<i64>,
}

#[derive(Deserialize)]
struct UpdateWatchlist {
    status: Option<String>,
    episodes_watched: Option<i64>,
    rating: Option<i64>,
}

#[derive(Deserialize)]
struct AddToWatchlist {
    mal_id: i64,
    title: String,
    image_url: String,
    status: String,
    episodes_watched: i64,
}
#[derive(Serialize)]
struct Stats {
    total: i64,
    completed: i64,
    watching: i64,
    plan_to_watch: i64,
    dropped: i64,
    average_rating: Option<f64>,
}

#[derive(Serialize, Deserialize)]
struct RecommendationEntry {
    entry: JikanAnime,
}

#[derive(Serialize, Deserialize)]
struct RecommendationsResponse {
    data: Vec<RecommendationEntry>,
}

#[get("/stats")]
async fn get_stats(pool: web::Data<SqlitePool>) -> HttpResponse {
    let row = sqlx::query!(
        "SELECT 
            COUNT(*) as total,
            COUNT(CASE WHEN status = 'Completed' THEN 1 END) as completed,
            COUNT(CASE WHEN status = 'Watching' THEN 1 END) as watching,
            COUNT(CASE WHEN status = 'Plan to Watch' THEN 1 END) as plan_to_watch,
            COUNT(CASE WHEN status = 'Dropped' THEN 1 END) as dropped,
            AVG(rating) as average_rating
        FROM watchlist"
    ).fetch_one(pool.get_ref()).await.unwrap();

    let stats = Stats {
            total: row.total,
            completed: row.completed,
            watching: row.watching,
            plan_to_watch: row.plan_to_watch,
            dropped: row.dropped,
            average_rating: row.average_rating,
        };

    HttpResponse::Ok().json(stats)
}

#[get("/recommendations/{mal_id}")]
async fn get_recommendations(path: web::Path<i64>) -> HttpResponse {
    let mal_id = path.into_inner();
    let url = format!("https://api.jikan.moe/v4/anime/{}/recommendations", mal_id);

    let response = reqwest::get(&url).await;

    match response {
        Ok(res) => {
            match res.json::<RecommendationsResponse>().await {
                Ok(data) => {
                    let anime: Vec<JikanAnime> = data.data.into_iter().take(6).map(|r| r.entry).collect();
                    HttpResponse::Ok().json(anime)
                }
                Err(_) => HttpResponse::InternalServerError().body("Failed to parse recommendations"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to reach Jikan API"),
    }
}

#[get("/search")]
async fn search_anime(query: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let q = query.get("q").cloned().unwrap_or_default();
    let genre = query.get("genre").cloned().unwrap_or_default();
    let page = query.get("page").cloned().unwrap_or("1".to_string());

   let mut url = format!(
    "https://api.jikan.moe/v4/anime?q={}&limit=10&sfw=true&page={}&order_by=members&sort=desc",
    q, page
);
    if !genre.is_empty() {
        url.push_str(&format!("&genres={}", genre));
    }

    let response = reqwest::get(&url).await;

    match response {
        Ok(res) => {
            match res.json::<JikanResponse>().await {
                Ok(data) => HttpResponse::Ok().json(data),
                Err(_) => HttpResponse::InternalServerError().body("Failed to parse response"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to reach Jikan API"),
    }
}

#[get("/watchlist")]
async fn get_watchlist(pool: web::Data<SqlitePool>) -> HttpResponse {
    let items = sqlx::query_as!(
        WatchlistItem,
       "SELECT id, mal_id, title, image_url, status, episodes_watched, rating FROM watchlist"
    )
    .fetch_all(pool.get_ref())
    .await;

    match items {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch watchlist"),
    }
}

#[actix_web::post("/watchlist")]
async fn add_watchlist(pool: web::Data<SqlitePool>, body: web::Json<AddToWatchlist>) -> HttpResponse {
    let result = sqlx::query!(
        "INSERT INTO watchlist (mal_id, title, image_url, status, episodes_watched) VALUES (?, ?, ?, ?, ?)",
        body.mal_id, body.title, body.image_url, body.status, body.episodes_watched
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Added!"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::put("/watchlist/{id}")]
async fn update_watchlist(pool: web::Data<SqlitePool>, path: web::Path<i64>, body: web::Json<UpdateWatchlist>) -> HttpResponse {
    let id = path.into_inner();

    if let Some(status) = &body.status {
        sqlx::query!("UPDATE watchlist SET status = ? WHERE id = ?", status, id)
            .execute(pool.get_ref())
            .await.ok();
    }

    if let Some(eps) = body.episodes_watched {
        sqlx::query!("UPDATE watchlist SET episodes_watched = ? WHERE id = ?", eps, id)
            .execute(pool.get_ref())
            .await.ok();
    }

    if let Some(rating) = body.rating {
        sqlx::query!("UPDATE watchlist SET rating = ? WHERE id = ?", rating, id)
            .execute(pool.get_ref())
            .await.ok();
    }

    HttpResponse::Ok().body("Updated!")
}

#[actix_web::delete("/watchlist/{id}")]
async fn delete_watchlist(pool: web::Data<SqlitePool>, path: web::Path<i64>) -> HttpResponse {
    let id = path.into_inner();
    sqlx::query!("DELETE FROM watchlist WHERE id = ?", id)
        .execute(pool.get_ref())
        .await.ok();
    HttpResponse::Ok().body("Deleted!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL not set");

    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Database connected!");
    println!("Server starting on http://localhost:8080");

    HttpServer::new(move || {
        App::new()
    .app_data(web::Data::new(pool.clone()))
    .service(search_anime)
    .service(get_watchlist)
    .service(add_watchlist)
    .service(update_watchlist)
    .service(delete_watchlist)
    .service(get_stats)
    .service(get_recommendations)
    .service(Files::new("/", "./static").index_file("index.html"))

})
.bind("127.0.0.1:8080")?
.run()
.await
}