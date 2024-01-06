use std::ops::Div;
use std::{thread, time};

use rdev::{simulate, Button, EventType, Key, SimulateError};
use serde::Deserialize;

use crate::app_context;

#[derive(Deserialize, Clone)]
pub struct MouseRequest {
    pub id: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta_x: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta_y: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<u8>,
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    thread::sleep(delay);

}

impl MouseRequest {
    fn mouse_down(self) {
        let key = match self.button.unwrap() {
            0 => Button::Left,
            1 => Button::Middle,
            2 => Button::Right,
            _ => Button::Unknown(0),
        };
        send(&EventType::ButtonPress(key));
    }

    fn mouse_up(self) {
         let key = match self.button.unwrap() {
            0 => Button::Left,
            1 => Button::Middle,
            2 => Button::Right,
            _ => Button::Unknown(0),
        };
        send(&EventType::ButtonRelease(key));
    }

    fn mouse_move(self) {
        let rate = app_context::HEIGHT.div(self.height.unwrap());
        let x = (self.x.unwrap() as f32 * rate) as f64;
        let y = (self.y.unwrap() as f32 * rate) as f64;
        send(&EventType::MouseMove { x, y });
    }

    fn mouse_wheel(self) {
        send(&EventType::Wheel { delta_x: -self.delta_x.unwrap(), delta_y: -self.delta_y.unwrap()});
    }

    pub fn process(self) {
        match self.id {
            // DOWN
            0 => {
                self.clone().mouse_move();
                self.mouse_down();
            }

            // UP
            1 => {
                self.clone().mouse_move();
                self.mouse_up();
            }

            // MOVE
            2 => {
                self.mouse_move();
            }

            // WHEEL
            3 => {
                self.clone().mouse_move();
                self.mouse_wheel();
            }

            _ => {
                print!("Incorrect request!");
            }
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct KeyRequest {
    pub id: u8,
    pub code: String,
}

impl KeyRequest {
    fn key_down(self) {
        send(&EventType::KeyPress(Self::from_key_code(self.code.as_str())));
    }

    fn key_up(self) {
        send(&EventType::KeyRelease(Self::from_key_code(self.code.as_str())));
    }

    pub fn process(self) {
        match self.id {
            // DOWN
            0 => {
                self.key_down();
            }

            // UP
            1 => {
                self.key_up();
            }

            _ => {
                print!("Incorrect request!");
            }
        }
    }

    fn from_key_code(code: &str) -> Key {
        match code {
            "altleft" | "altright" => Key::Alt,
            "altgraph" => Key::AltGr,
            "backspace" => Key::Backspace,
            "capslock" => Key::CapsLock,
            "controlleft" => Key::ControlLeft,
            "controlright" => Key::ControlRight,
            "delete" => Key::Delete,
            "arrowdown" => Key::DownArrow,
            "end" => Key::End,
            "escape" => Key::Escape,
            "f1" => Key::F1,
            "f10" => Key::F10,
            "f11" => Key::F11,
            "f12" => Key::F12,
            "f2" => Key::F2,
            "f3" => Key::F3,
            "f4" => Key::F4,
            "f5" => Key::F5,
            "f6" => Key::F6,
            "f7" => Key::F7,
            "f8" => Key::F8,
            "f9" => Key::F9,
            "home" => Key::Home,
            "arrowleft" => Key::LeftArrow,
            "metaleft" => Key::MetaLeft,
            "metaright" => Key::MetaRight,
            "pagedown" => Key::PageDown,
            "pageup" => Key::PageUp,
            "enter" => Key::Return,
            "arrowright" => Key::RightArrow,
            "shiftleft" => Key::ShiftLeft,
            "shiftright" => Key::ShiftRight,
            "space" => Key::Space,
            "tab" => Key::Tab,
            "arrowup" => Key::UpArrow,
            "printscreen" => Key::PrintScreen,
            "scrolllock" => Key::ScrollLock,
            "pause" => Key::Pause,
            "numlock" => Key::NumLock,
            "backquote" => Key::BackQuote,
            "digit1" => Key::Num1,
            "digit2" => Key::Num2,
            "digit3" => Key::Num3,
            "digit4" => Key::Num4,
            "digit5" => Key::Num5,
            "digit6" => Key::Num6,
            "digit7" => Key::Num7,
            "digit8" => Key::Num8,
            "digit9" => Key::Num9,
            "digit0" => Key::Num0,
            "minus" => Key::Minus,
            "equal" => Key::Equal,
            "keyq" => Key::KeyQ,
            "keyw" => Key::KeyW,
            "keye" => Key::KeyE,
            "keyr" => Key::KeyR,
            "keyt" => Key::KeyT,
            "keyy" => Key::KeyY,
            "keyu" => Key::KeyU,
            "keyi" => Key::KeyI,
            "keyo" => Key::KeyO,
            "keyp" => Key::KeyP,
            "bracketleft" => Key::LeftBracket,
            "bracketright" => Key::RightBracket,
            "keya" => Key::KeyA,
            "keys" => Key::KeyS,
            "keyd" => Key::KeyD,
            "keyf" => Key::KeyF,
            "keyg" => Key::KeyG,
            "keyh" => Key::KeyH,
            "keyj" => Key::KeyJ,
            "keyk" => Key::KeyK,
            "keyl" => Key::KeyL,
            "semicolon" => Key::SemiColon,
            "quote" => Key::Quote,
            "backslash" => Key::BackSlash,
            "intlbackslash" => Key::IntlBackslash,
            "keyz" => Key::KeyZ,
            "keyx" => Key::KeyX,
            "keyc" => Key::KeyC,
            "keyv" => Key::KeyV,
            "keyb" => Key::KeyB,
            "keyn" => Key::KeyN,
            "keym" => Key::KeyM,
            "comma" => Key::Comma,
            "period" => Key::Dot,
            "slash" => Key::Slash,
            "insert" => Key::Insert,
            "numpadenter" => Key::KpReturn,
            "numpadsubtract" => Key::KpMinus,
            "numpadadd" => Key::KpPlus,
            "numpadmultiply" => Key::KpMultiply,
            "numpaddivide" => Key::KpDivide,
            "numpad0" => Key::Kp0,
            "numpad1" => Key::Kp1,
            "numpad2" => Key::Kp2,
            "numpad3" => Key::Kp3,
            "numpad4" => Key::Kp4,
            "numpad5" => Key::Kp5,
            "numpad6" => Key::Kp6,
            "numpad7" => Key::Kp7,
            "numpad8" => Key::Kp8,
            "numpad9" => Key::Kp9,
            "numpaddelete" => Key::KpDelete,
            "f" => Key::Function,
            _ => Key::Unknown(0),
        }
    }
}