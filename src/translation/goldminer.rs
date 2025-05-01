use super::Language;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref GOLDMINER_TRANSLATIONS: HashMap<&'static str, HashMap<Language, String>> = {
        let mut m = HashMap::new();

        let mut goldminer_title = HashMap::new();
        goldminer_title.insert(Language::English, "Gold Miner".to_string());
        goldminer_title.insert(Language::Chinese, "黄金矿工".to_string());
        m.insert("goldminer.goldminer_title", goldminer_title);

        let mut level = HashMap::new();
        level.insert(Language::English, "Level:".to_string());
        level.insert(Language::Chinese, "关卡：".to_string());
        m.insert("goldminer.level", level);

        let mut welcome_to = HashMap::new();
        welcome_to.insert(Language::English, "Welcome to".to_string());
        welcome_to.insert(Language::Chinese, "欢迎来到".to_string());
        m.insert("goldminer.welcome_to", welcome_to);

        // ... 添加所有黄金矿工相关的翻译
        let mut how_to_play = HashMap::new();
        how_to_play.insert(Language::English, "How to Play:".to_string());
        how_to_play.insert(Language::Chinese, "游戏说明：".to_string());
        m.insert("goldminer.how_to_play", how_to_play);

        let mut hook_swing = HashMap::new();
        hook_swing.insert(Language::English, "1. The hook swings automatically".to_string());
        hook_swing.insert(Language::Chinese, "1. 钩子自动摆动".to_string());
        m.insert("goldminer.hook_swing", hook_swing);

        let mut press_space = HashMap::new();
        press_space.insert(Language::English, "2. Press SPACE to release the hook".to_string());
        press_space.insert(Language::Chinese, "2. 按空格键释放钩子".to_string());
        m.insert("goldminer.press_space", press_space);

        let mut catch_gold = HashMap::new();
        catch_gold.insert(Language::English, "3. Catch gold (◆/♦) to earn points:".to_string());
        catch_gold.insert(Language::Chinese, "3. 抓取金块（◆/♦）获得分数：".to_string());
        m.insert("goldminer.catch_gold", catch_gold);

        let mut big_gold_points = HashMap::new();
        big_gold_points.insert(Language::English, "   - Big gold (◆) : 200 points".to_string());
        big_gold_points.insert(Language::Chinese, "   - 大金块（◆）：200分".to_string());
        m.insert("goldminer.big_gold_points", big_gold_points);

        let mut small_gold_points = HashMap::new();
        small_gold_points.insert(Language::English, "   - Small gold (♦) : 100 points".to_string());
        small_gold_points.insert(Language::Chinese, "   - 小金块（♦）：100分".to_string());
        m.insert("goldminer.small_gold_points", small_gold_points);

        let mut avoid_stones = HashMap::new();
        avoid_stones.insert(Language::English, "4. Avoid stones (■/□) (-50 points)".to_string());
        avoid_stones.insert(Language::Chinese, "4. 避开石头（■/□）（-50分）".to_string());
        m.insert("goldminer.avoid_stones", avoid_stones);

        let mut collect_all_gold = HashMap::new();
        collect_all_gold.insert(Language::English, "5. Collect all gold to advance to next level".to_string());
        collect_all_gold.insert(Language::Chinese, "5. 收集所有金块进入下一关".to_string());
        m.insert("goldminer.collect_all_gold", collect_all_gold);

        let mut higher_levels = HashMap::new();
        higher_levels.insert(Language::English, "6. Higher Levels:".to_string());
        higher_levels.insert(Language::Chinese, "6. 更高关卡：".to_string());
        m.insert("goldminer.higher_levels", higher_levels);

        let mut faster_hook = HashMap::new();
        faster_hook.insert(Language::English, "- Hook swings faster".to_string());
        faster_hook.insert(Language::Chinese, "- 钩子摆动更快".to_string());
        m.insert("goldminer.faster_hook", faster_hook);

        let mut heavier_items = HashMap::new();
        heavier_items.insert(Language::English, "- Items become heavier".to_string());
        heavier_items.insert(Language::Chinese, "- 物品变得更重".to_string());
        m.insert("goldminer.heavier_items", heavier_items);

        let mut more_obstacles = HashMap::new();
        more_obstacles.insert(Language::English, "- More obstacles appear".to_string());
        more_obstacles.insert(Language::Chinese, "- 出现更多障碍".to_string());
        m.insert("goldminer.more_obstacles", more_obstacles);

        let mut controls_title = HashMap::new();
        controls_title.insert(Language::English, "Game Controls:".to_string());
        controls_title.insert(Language::Chinese, "游戏控制：".to_string());
        m.insert("goldminer.controls_title", controls_title);

        let mut space_control = HashMap::new();
        space_control.insert(Language::English, "SPACE: Release/retract hook".to_string());
        space_control.insert(Language::Chinese, "空格键：释放/收回钩子".to_string());
        m.insert("goldminer.space_control", space_control);

        let mut quit_control = HashMap::new();
        quit_control.insert(Language::English, "Q: Return to main menu".to_string());
        quit_control.insert(Language::Chinese, "Q：返回主菜单".to_string());
        m.insert("goldminer.quit_control", quit_control);

        let mut press_enter = HashMap::new();
        press_enter.insert(Language::English, "Press ENTER to start!".to_string());
        press_enter.insert(Language::Chinese, "按回车键开始！".to_string());
        m.insert("goldminer.press_enter", press_enter);

        m


    };
}
