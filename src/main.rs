use dirs::home_dir;
use reqwest::Error;
use serde::Deserialize;
use std::fs::File;
use std::io::{self, Write};

#[derive(Deserialize)]
struct Post {
    file_url: String,
}

#[derive(Deserialize)]
struct Response {
    post: Option<Vec<Post>>, // Используем Option для обработки отсутствия поля
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Запрашиваем у пользователя ввод тегов
    let mut tags = String::new();
    println!("Введите теги (например, skirt highres):");
    io::stdin().read_line(&mut tags)?;
    let tags = tags.trim(); // Убираем лишние пробелы и символы новой строки

    let mut all_file_urls = Vec::new();

    for pid in 0..100 {
        let url = format!(
            "https://gelbooru.com/index.php?page=dapi&s=post&q=index&tags={}&json=1&pid={}",
            tags, pid
        );
        let response: Response = reqwest::get(&url).await?.json().await?;

        // Проверяем, есть ли посты
        if let Some(posts) = response.post {
            for post in posts {
                all_file_urls.push(post.file_url);
            }
        }
    }

    // Формируем имя файла на основе тегов
    let safe_tags = tags.replace(" ", "_"); // Заменяем пробелы на подчеркивания
    let home_dir = home_dir().ok_or("Невозмоно найти директорию")?;
    let file_path = home_dir.join(format!("file_urls_{}.txt", safe_tags));
    let mut file = File::create(&file_path)?;

    for url in all_file_urls {
        writeln!(file, "{}", url)?;
    }

    println!("Ссылки записаны в: {:?}", file_path);
    Ok(())
}
