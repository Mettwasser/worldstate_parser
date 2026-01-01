#[derive(Debug, Clone, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Region {
    pub unique_name: String,

    pub name: String,

    pub system_index: i64,

    #[serde(rename = "systemName")]
    pub planet: String,

    pub node_type: i64,

    pub mastery_req: i64,

    pub mission_index: i64,

    pub faction_index: i64,

    pub min_enemy_level: i64,

    pub max_enemy_level: i64,
}
