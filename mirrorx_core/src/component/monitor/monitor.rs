#[derive(Debug)]
pub struct Monitor {
    pub id: String,
    pub name: String,
    pub refresh_rate: u8,
    pub width: u16,
    pub height: u16,
    pub is_primary: bool,
    pub screen_shot: Vec<u8>,
}
