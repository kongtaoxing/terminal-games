use super::Language;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SNAKE_TRANSLATIONS: HashMap<&'static str, HashMap<Language, String>> = {
        let mut m = HashMap::new();

        let mut snake_title = HashMap::new();
        snake_title.insert(Language::English, "Snake".to_string());
        snake_title.insert(Language::Chinese, "贪吃蛇".to_string());
        m.insert("snake_title", snake_title);

        let mut score = HashMap::new();
        score.insert(Language::English, "Score:".to_string());
        score.insert(Language::Chinese, "分数：".to_string());
        m.insert("score", score);

        let mut game_over = HashMap::new();
        game_over.insert(Language::English, "Game Over!".to_string());
        game_over.insert(Language::Chinese, "游戏结束！".to_string());
        m.insert("game_over", game_over);

        let mut press_r_restart = HashMap::new();
        press_r_restart.insert(Language::English, "Press 'R' to restart".to_string());
        press_r_restart.insert(Language::Chinese, "按 'R' 键重新开始".to_string());
        m.insert("press_r_restart", press_r_restart);

        let mut how_to_play = HashMap::new();
        how_to_play.insert(Language::English, "How to Play:".to_string());
        how_to_play.insert(Language::Chinese, "游戏说明：".to_string());
        m.insert("how_to_play", how_to_play);

        let mut move_snake = HashMap::new();
        move_snake.insert(Language::English, "Use arrow keys or WASD to move the snake".to_string());
        move_snake.insert(Language::Chinese, "使用方向键或 WASD 移动蛇".to_string());
        m.insert("move_snake", move_snake);

        let mut eat_food_title = HashMap::new();
        eat_food_title.insert(
            Language::English, 
            "Eat food to grow longer and score points:".to_string()
        );
        eat_food_title.insert(
            Language::Chinese, 
            "吃掉食物可以变长并得分：".to_string()
        );
        m.insert("eat_food_title", eat_food_title);

        let mut apple_desc = HashMap::new();
        apple_desc.insert(
            Language::English, 
            "- Apple (🍎): 50 points (large)".to_string()
        );
        apple_desc.insert(
            Language::Chinese, 
            "- 苹果(🍎)：50分（大食物）".to_string()
        );
        m.insert("apple_desc", apple_desc);

        let mut candy_desc = HashMap::new();
        candy_desc.insert(
            Language::English, 
            "- Candy (🍬): 150 points (small)".to_string()
        );
        candy_desc.insert(
            Language::Chinese, 
            "- 糖果(🍬)：150分（小食物）".to_string()
        );
        m.insert("candy_desc", candy_desc);

        let mut avoid_walls = HashMap::new();
        avoid_walls.insert(
            Language::English, 
            "Avoid walls, yourself, and be careful with large apples!".to_string()
        );
        avoid_walls.insert(
            Language::Chinese, 
            "避免撞到墙壁和自己，注意大苹果占据的空间！".to_string()
        );
        m.insert("avoid_walls", avoid_walls);

        let mut press_enter = HashMap::new();
        press_enter.insert(Language::English, "Press ENTER to start".to_string());
        press_enter.insert(Language::Chinese, "按回车键开始游戏".to_string());
        m.insert("press_enter", press_enter);

        let mut pause_game = HashMap::new();
        pause_game.insert(Language::English, "Press P/ESC to pause".to_string());
        pause_game.insert(Language::Chinese, "按 P/ESC 键暂停游戏".to_string());
        m.insert("pause_game", pause_game);

        m
    };
}
