-- Add migration script here
CREATE TABLE watchlist (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mal_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    image_url TEXT,
    status TEXT NOT NULL DEFAULT 'Plan to Watch',
    episodes_watched INTEGER NOT NULL DEFAULT 0
);