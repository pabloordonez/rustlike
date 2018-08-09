use core::events::event::{Key, KeyboardEvent, KeyboardEventType, MouseEvent};
use core::point_2d::Point2d;

pub struct MouseState {
    pub left_button: bool,
    pub middle_button: bool,
    pub right_button: bool,
    pub extra_button_1: bool,
    pub extra_button_2: bool,
    pub extra_button_3: bool,
    pub extra_button_4: bool,
    pub position: Point2d,
}

pub struct KeyboardState {
    pub keys: [bool; 175],
}

impl MouseState {
    pub fn new() -> MouseState {
        MouseState {
            left_button: false,
            middle_button: false,
            right_button: false,
            extra_button_1: false,
            extra_button_2: false,
            extra_button_3: false,
            extra_button_4: false,
            position: Point2d::empty(),
        }
    }

    pub fn update_from_event(&mut self, mouse: MouseEvent) {
        self.left_button = mouse.left_button;
        self.middle_button = mouse.middle_button;
        self.right_button = mouse.right_button;
        self.extra_button_1 = mouse.extra_button_1;
        self.extra_button_2 = mouse.extra_button_2;
        self.extra_button_3 = mouse.extra_button_3;
        self.extra_button_4 = mouse.extra_button_4;
        self.position = mouse.position;
    }
}

impl KeyboardState {
    pub fn new() -> KeyboardState {
        KeyboardState { keys: [false; 175] }
    }

    pub fn update_from_event(&mut self, keyboard: KeyboardEvent) {
        self.keys[keyboard.key.to_u32() as usize] =
            keyboard.event_type == KeyboardEventType::KeyDown;
        self.keys[Key::LeftShift.to_u32() as usize] = keyboard.left_shift;
        self.keys[Key::LeftControl.to_u32() as usize] = keyboard.left_control;
        self.keys[Key::LeftMenu.to_u32() as usize] = keyboard.left_menu;
        self.keys[Key::RightShift.to_u32() as usize] = keyboard.right_shift;
        self.keys[Key::RightControl.to_u32() as usize] = keyboard.right_control;
        self.keys[Key::RightMenu.to_u32() as usize] = keyboard.right_menu;
    }
}
