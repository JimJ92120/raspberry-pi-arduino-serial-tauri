#[derive(Debug)]
pub struct State {
    pub light: bool,
}
impl State {
    pub fn new() -> Self {
        State {
            light: false,
        }
    }
}
