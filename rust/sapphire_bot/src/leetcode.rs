use reqwest::Client;
use serde_json::json;

pub async fn get_problem(slug: &str) -> Option<(String, String)>{
    let client = Client::new();

    let query = json!({
    "operationName": "questionDetail",
    "query": "query questionDetail($titleSlug: String!) { question(titleSlug: $titleSlug) { title difficulty questionFrontendId } }",
    "variables": { "titleSlug": slug }
});

    let response = client
        .post("https://leetcode.com/graphql")
        .json(&query)
        .send()
        .await
        .ok()?;

    let data: serde_json::Value = response.json().await.ok()?;
    let title = data["data"]["question"]["title"].as_str()?;
    let difficulty = data["data"]["question"]["difficulty"].as_str()?;

    Some((title.to_string(), difficulty.to_string()))
}