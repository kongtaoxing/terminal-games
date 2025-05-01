mod common;
mod goldminer;
mod snake;
mod tetris;
mod twenty_forty_eight;
mod minesweeper;
pub use common::Language;

use common::COMMON_TRANSLATIONS;
use goldminer::GOLDMINER_TRANSLATIONS;
use snake::SNAKE_TRANSLATIONS;
use std::collections::HashMap;
use tetris::TETRIS_TRANSLATIONS;
use twenty_forty_eight::TWENTY_FORTY_EIGHT_TRANSLATIONS;
use minesweeper::MINESWEEPER_TRANSLATIONS;
pub struct Translations {
    texts: HashMap<String, HashMap<Language, String>>,
    current_language: Language,
    namespace: String,
}

impl Translations {
    fn detect_system_language() -> Language {
        if let Ok(lang) = std::env::var("LANG") {
            let lang = lang.to_lowercase();
            if lang.starts_with("zh_") {
                return Language::Chinese;
            }
            // 可以根据需要添加更多语言判断
        }
        Language::English // 默认返回英语
    }

    pub fn new() -> Self {
        let mut texts = HashMap::new();

        // 加载所有翻译
        texts.extend(
            COMMON_TRANSLATIONS
                .iter()
                .map(|(k, v)| (k.to_string(), v.clone())),
        );
        texts.extend(
            GOLDMINER_TRANSLATIONS
                .iter()
                .map(|(k, v)| (k.to_string(), v.clone())),
        );
        texts.extend(
            TETRIS_TRANSLATIONS
                .iter()
                .map(|(k, v)| (k.to_string(), v.clone())),
        );
        texts.extend(
            SNAKE_TRANSLATIONS
                .iter()
                .map(|(k, v)| (k.to_string(), v.clone())),
        );
        texts.extend(
            TWENTY_FORTY_EIGHT_TRANSLATIONS
                .iter()
                .map(|(k, v)| (k.to_string(), v.clone())),
        );
        texts.extend(
            MINESWEEPER_TRANSLATIONS
                .iter()
                .map(|(k, v)| (k.to_string(), v.clone())),
        );
        Self {
            texts,
            current_language: Self::detect_system_language(),
            namespace: String::new(),
        }
    }

    pub fn get_text(&self, key: &str) -> String {
        let full_key = if self.namespace.is_empty() {
            key.to_string()
        } else {
            format!("{}.{}", self.namespace, key)
        };

        self.texts
            .get(&full_key)
            .and_then(|translations| {
                translations
                    .get(&self.current_language)
                    .or_else(|| translations.get(&Language::English))
            })
            .cloned()
            .unwrap_or_else(|| format!("Missing translation: {}", full_key))
    }

    pub fn with_namespace(mut self, namespace: &str) -> Self {
        self.namespace = namespace.to_string();
        self
    }

    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
    }

    pub fn current_language(&self) -> Language {
        self.current_language
    }
}
