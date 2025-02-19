use super::Language;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TWENTY_FORTY_EIGHT_TRANSLATIONS: HashMap<&'static str, HashMap<Language, String>> = {
        let mut m = HashMap::new();

        let mut game_title = HashMap::new();
        game_title.insert(Language::English, "2048".to_string());
        game_title.insert(Language::Chinese, "2048".to_string());
        m.insert("twenty_forty_eight_title", game_title);

        let mut welcome_title = HashMap::new();
        welcome_title.insert(Language::English, "Welcome to 2048!".to_string());
        welcome_title.insert(Language::Chinese, "欢迎来到2048！".to_string());
        m.insert("welcome_title", welcome_title);

        let mut press_enter = HashMap::new();
        press_enter.insert(Language::English, "Press Enter to start".to_string());
        press_enter.insert(Language::Chinese, "按回车键开始".to_string());
        m.insert("press_enter", press_enter);

        let mut score = HashMap::new();
        score.insert(Language::English, "Score: ".to_string());
        score.insert(Language::Chinese, "分数：".to_string());
        m.insert("score", score);

        let mut game_over = HashMap::new();
        game_over.insert(Language::English, "Game Over! Press 'r' to restart".to_string());
        game_over.insert(Language::Chinese, "游戏结束！按'r'键重新开始".to_string());
        m.insert("game_over", game_over);

        let mut how_to_play = HashMap::new();
        how_to_play.insert(Language::English, "How to Play:".to_string());
        how_to_play.insert(Language::Chinese, "游戏说明：".to_string());
        m.insert("how_to_play", how_to_play);

        let mut move_controls = HashMap::new();
        move_controls.insert(
            Language::English, 
            "Use Arrow keys or WASD to move tiles".to_string()
        );
        move_controls.insert(
            Language::Chinese, 
            "使用方向键或WASD移动方块".to_string()
        );
        m.insert("move_controls", move_controls);

        let mut merge_tip = HashMap::new();
        merge_tip.insert(
            Language::English, 
            "Merge same numbers to get 2048!".to_string()
        );
        merge_tip.insert(
            Language::Chinese, 
            "合并相同数字以获得2048！".to_string()
        );
        m.insert("merge_tip", merge_tip);

        let mut pause_control = HashMap::new();
        pause_control.insert(
            Language::English, 
            "Press P or ESC to pause game".to_string()
        );
        pause_control.insert(
            Language::Chinese, 
            "按P或ESC键暂停游戏".to_string()
        );
        m.insert("pause_control", pause_control);

        let mut game_paused = HashMap::new();
        game_paused.insert(Language::English, "Game Paused".to_string());
        game_paused.insert(Language::Chinese, "游戏暂停".to_string());
        m.insert("game_paused", game_paused);

        let mut resume_tip = HashMap::new();
        resume_tip.insert(
            Language::English, 
            "Press P or ESC to resume".to_string()
        );
        resume_tip.insert(
            Language::Chinese, 
            "按P或ESC键继续".to_string()
        );
        m.insert("resume_tip", resume_tip);

        m
    };
}
