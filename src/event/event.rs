
#[derive(Debug, Clone, Copy)]
pub struct EventDispatcher {
    event: Event,
    handled: bool,
}

impl EventDispatcher {
    pub fn new(event: Event) -> Self {
        Self {event, handled: false}
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WindowEvent{
    Close,
    Resize{width: u32, height: u32},
    Focus,
    LostFocus,
    Moved,
}

#[derive(Debug, Clone, Copy)]
pub enum AppEvent{
    Tick,
    Update,
    Render,
}

#[derive(Debug, Clone, Copy)]
pub enum KeyEvent{
    Pressed{keycode: i32, repeat_count: i32},
    Released{keycode: i32},
}

#[derive(Debug, Clone, Copy)]
pub enum MouseEvent{
    Pressed{x: f32, y: f32},
    Released{x: f32, y: f32},
    Moved{x: f32, y: f32},
    Scolled{x: f32, y: f32},
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Window(WindowEvent),
    App(AppEvent),
    Key(KeyEvent),
    Mouse(MouseEvent),
}

impl Event {
    pub fn is_input(&self) -> bool {
        match *self {
            Event::Window(_) => true,
            Event::Key(_) => true,
            Event::Mouse(_) => true,
            Event::App(_) => false,
        }
    }
}


