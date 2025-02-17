use super::Language;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TETRIS_TRANSLATIONS: HashMap<&'static str, HashMap<Language, String>> = {
        let mut m = HashMap::new();

        let mut tetris_title = HashMap::new();
        tetris_title.insert(Language::English, "Tetris".to_string());
        tetris_title.insert(Language::Chinese, "俄罗斯方块".to_string());
        m.insert("tetris_title", tetris_title);

        let mut score = HashMap::new();
        score.insert(Language::English, "Score:".to_string());
        score.insert(Language::Chinese, "分数：".to_string());
        m.insert("score", score);

        let mut game_over = HashMap::new();
        game_over.insert(Language::English, "Game Over!".to_string());
        game_over.insert(Language::Chinese, "游戏结束！".to_string());
        m.insert("game_over", game_over);

        m
    };
} 