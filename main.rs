use bevy::prelude::*;
use std::fs;
use std::env;
use toml;

fn read_nick() -> Option<String> {
     let current_dir = env::current_dir().expect("Unable to get current directory");
      println!("Current directory: {}", current_dir.display());
    // Чтение содержимого файла
    let content = fs::read_to_string("profile.toml").expect("Unable to read file");

    // Парсим TOML данные
    let nick: toml::Value = toml::de::from_str(&content).expect("Failed to parse TOML");

    // Извлекаем значение поля "nickname"
    if let Some(title) = nick.get("nickname") {
        // Возвращаем строку, если поле найдено
        return title.as_str().map(|s| s.to_string());
    } else {
        // Если поле не найдено, выводим сообщение
        println!("Поле 'nickname' не было найдено!");
        return None;
    }
}

fn main() {
    let mut gamenick = read_nick();
    let mut tinky = App::new();
    tinky.add_plugins(DefaultPlugins);
    tinky.run();
}
