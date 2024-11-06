use rusqlite::Connection;

use super::Recipe;

pub const DATABASE_FILENAME: &str = "recipes.db";

pub fn create_recipe_table(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS recipes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            url TEXT,
            image_url TEXT,
            calories TEXT,
            cook_time TEXT,
            portions TEXT,
            protein TEXT,
            fat TEXT,
            carbohydrates TEXT,
            ingredients TEXT
        )",
        [],
    ).unwrap();
}

pub fn insert_recipe(conn: &Connection, recipe: &Recipe) {
    conn.execute(
        "INSERT INTO recipes (name, url, image_url, calories, cook_time, portions, protein, fat, carbohydrates, ingredients) 
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        [
            &recipe.name,
            &recipe.url,
            recipe.image_url.as_ref().map(String::as_str).unwrap_or(""),
            recipe.calories.as_ref().map(String::as_str).unwrap_or(""),
            recipe.cook_time.as_ref().map(String::as_str).unwrap_or(""),
            recipe.portions.as_ref().map(String::as_str).unwrap_or(""),
            recipe.protein.as_ref().map(String::as_str).unwrap_or(""),
            recipe.fat.as_ref().map(String::as_str).unwrap_or(""),
            recipe.carbohydrates.as_ref().map(String::as_str).unwrap_or(""),
            &recipe.ingredients.join(", "),
        ],
    ).unwrap();
}