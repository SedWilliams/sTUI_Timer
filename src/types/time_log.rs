#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TimeLog {
    pub id: u32,
    pub time_spent: [i8;3],
    pub date: String,
    //pub date: chrono::format::DelayedFormat<chrono::format::StrftimeItems<'static>>, //type from
                                                                                     //chrono crate
}
