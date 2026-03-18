use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HomeProfile {
    pub current_identity: String,  // 比如 "Home Page"
    pub avatar_url: String,        // 头像地址
    pub bg_url: String,            // 背景图地址
    pub team_members: Vec<String>, // ["Suly", "chixiaoshu"]
    pub site_version: String,      // 版本号
    pub intro: String,             // 自我介绍
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,  // "ok"
    pub version: &'static str, // env!("CARGO_PKG_VERSION")
}

impl Default for HomeProfile {
    fn default() -> Self {
        Self {
            current_identity: "Home Page - Default".to_string(),
            avatar_url: "/images/avatar.webp".to_string(),
            bg_url: "/images/bg.png".to_string(),
            team_members: vec!["Suly".into(), "chixiaoshu".into(), "Azureus".into()],
            site_version: "0.1.0-alpha".to_string(),
            intro: "Hi, 这里是 梨。\n欢迎下滑探索我的项目～".to_string(),
        }
    }
}

impl Default for HealthResponse {
    fn default() -> Self {
        Self {
            status: "ok",
            version: "CARGO_PKG_VERSION",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub desc: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectList {
    pub items: Vec<Project>,
}

// 为 ProjectList 实现 Default
impl Default for ProjectList {
    fn default() -> Self {
        Self {
            items: vec![Project {
                name: "更多项目".into(),
                desc: "探索我的 GitHub 获取更多有趣的工具。".into(),
                url: "https://github.com/chixiaoshu".into(),
            }],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AboutItem {
    pub icon_url: String,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AboutList {
    pub items: Vec<AboutItem>,
}

impl Default for AboutList {
    fn default() -> Self {
        Self {
            items: vec![AboutItem {
                icon_url: "/images/default_icon.webp".into(),
                title: "Waiting for ?".into(),
                content: "default.".into(),
            }],
        }
    }
}
