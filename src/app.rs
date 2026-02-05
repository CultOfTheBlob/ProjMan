#[derive(Debug)]
pub enum CurrentScreen
{
    Main,
}

#[derive(Debug)]
pub struct App
{
    pub current_screen: CurrentScreen,
}

impl App
{
    pub fn new() -> App
    {
        App {
            current_screen: CurrentScreen::Main,
        }
    }
}

