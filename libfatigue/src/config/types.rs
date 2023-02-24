use serde_yaml::Value;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub struct FatigueTesterConfig {
    #[serde(rename = "run")]
    pub run_info: FatigueTesterRunInformation,
    pub actions: Vec<FatigueTesterConfigAction>,
    #[serde(default)]
    pub static_context: Vec<FatigueStaticContextAction>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FatigueTesterConfigAction {
    #[serde(rename = "type")]
    pub action_type: String,

    #[serde(rename = "properties")]
    pub action_properties: Value,

    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FatigueStaticContextAction {
    #[serde(rename = "type")]
    pub context_action_type: String,

    #[serde(rename = "properties")]
    pub properties: Value,

    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FatigueTesterRunInformation {
    pub base_url: String,
    #[serde(with = "serde_yaml::with::singleton_map")]
    pub duration: RunDuration,
    #[serde(default)]
    pub concurrency: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RunDuration {
    #[serde(rename = "iteration")]
    Iteration {
        #[serde(default, with = "humantime_serde")]
        warm_up: Option<Duration>,
        #[serde(default)]
        iterations: u64,
    },
    #[serde(rename = "timed")]
    Timed {
        #[serde(with = "humantime_serde")]
        duration: Duration,
        #[serde(default, with = "humantime_serde")]
        warm_up: Option<Duration>,
    },
}
