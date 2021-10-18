use glutin::event::VirtualKeyCode;
use mlua::prelude::*;
use mlua::UserData;
use std::any::type_name;

fn keycode_to_str(keycode: VirtualKeyCode) -> &'static str {
    match keycode {
        VirtualKeyCode::Key1 => "Key1",
        VirtualKeyCode::Key2 => "Key2",
        VirtualKeyCode::Key3 => "Key3",
        VirtualKeyCode::Key4 => "Key4",
        VirtualKeyCode::Key5 => "Key5",
        VirtualKeyCode::Key6 => "Key6",
        VirtualKeyCode::Key7 => "Key7",
        VirtualKeyCode::Key8 => "Key8",
        VirtualKeyCode::Key9 => "Key9",
        VirtualKeyCode::Key0 => "Key0",
        VirtualKeyCode::A => "A",
        VirtualKeyCode::B => "B",
        VirtualKeyCode::C => "C",
        VirtualKeyCode::D => "D",
        VirtualKeyCode::E => "E",
        VirtualKeyCode::F => "F",
        VirtualKeyCode::G => "G",
        VirtualKeyCode::H => "H",
        VirtualKeyCode::I => "I",
        VirtualKeyCode::J => "J",
        VirtualKeyCode::K => "K",
        VirtualKeyCode::L => "L",
        VirtualKeyCode::M => "M",
        VirtualKeyCode::N => "N",
        VirtualKeyCode::O => "O",
        VirtualKeyCode::P => "P",
        VirtualKeyCode::Q => "Q",
        VirtualKeyCode::R => "R",
        VirtualKeyCode::S => "S",
        VirtualKeyCode::T => "T",
        VirtualKeyCode::U => "U",
        VirtualKeyCode::V => "V",
        VirtualKeyCode::W => "W",
        VirtualKeyCode::X => "X",
        VirtualKeyCode::Y => "Y",
        VirtualKeyCode::Z => "Z",
        VirtualKeyCode::Escape => "Escape",
        VirtualKeyCode::F1 => "F1",
        VirtualKeyCode::F2 => "F2",
        VirtualKeyCode::F3 => "F3",
        VirtualKeyCode::F4 => "F4",
        VirtualKeyCode::F5 => "F5",
        VirtualKeyCode::F6 => "F6",
        VirtualKeyCode::F7 => "F7",
        VirtualKeyCode::F8 => "F8",
        VirtualKeyCode::F9 => "F9",
        VirtualKeyCode::F10 => "F10",
        VirtualKeyCode::F11 => "F11",
        VirtualKeyCode::F12 => "F12",
        VirtualKeyCode::F13 => "F13",
        VirtualKeyCode::F14 => "F14",
        VirtualKeyCode::F15 => "F15",
        VirtualKeyCode::F16 => "F16",
        VirtualKeyCode::F17 => "F17",
        VirtualKeyCode::F18 => "F18",
        VirtualKeyCode::F19 => "F19",
        VirtualKeyCode::F20 => "F20",
        VirtualKeyCode::F21 => "F21",
        VirtualKeyCode::F22 => "F22",
        VirtualKeyCode::F23 => "F23",
        VirtualKeyCode::F24 => "F24",
        VirtualKeyCode::Snapshot => "Snapshot",
        VirtualKeyCode::Scroll => "Scroll",
        VirtualKeyCode::Pause => "Pause",
        VirtualKeyCode::Insert => "Insert",
        VirtualKeyCode::Home => "Home",
        VirtualKeyCode::Delete => "Delete",
        VirtualKeyCode::End => "End",
        VirtualKeyCode::PageDown => "PageDown",
        VirtualKeyCode::PageUp => "PageUp",
        VirtualKeyCode::Left => "Left",
        VirtualKeyCode::Up => "Up",
        VirtualKeyCode::Right => "Right",
        VirtualKeyCode::Down => "Down",
        VirtualKeyCode::Back => "Back",
        VirtualKeyCode::Return => "Return",
        VirtualKeyCode::Space => "Space",
        VirtualKeyCode::Compose => "Compose",
        VirtualKeyCode::Caret => "Caret",
        VirtualKeyCode::Numlock => "Numlock",
        VirtualKeyCode::Numpad0 => "Numpad0",
        VirtualKeyCode::Numpad1 => "Numpad1",
        VirtualKeyCode::Numpad2 => "Numpad2",
        VirtualKeyCode::Numpad3 => "Numpad3",
        VirtualKeyCode::Numpad4 => "Numpad4",
        VirtualKeyCode::Numpad5 => "Numpad5",
        VirtualKeyCode::Numpad6 => "Numpad6",
        VirtualKeyCode::Numpad7 => "Numpad7",
        VirtualKeyCode::Numpad8 => "Numpad8",
        VirtualKeyCode::Numpad9 => "Numpad9",
        VirtualKeyCode::NumpadAdd => "NumpadAdd",
        VirtualKeyCode::NumpadDivide => "NumpadDivide",
        VirtualKeyCode::NumpadDecimal => "NumpadDecimal",
        VirtualKeyCode::NumpadComma => "NumpadComma",
        VirtualKeyCode::NumpadEnter => "NumpadEnter",
        VirtualKeyCode::NumpadEquals => "NumpadEquals",
        VirtualKeyCode::NumpadMultiply => "NumpadMultiply",
        VirtualKeyCode::NumpadSubtract => "NumpadSubtract",
        VirtualKeyCode::AbntC1 => "AbntC1",
        VirtualKeyCode::AbntC2 => "AbntC2",
        VirtualKeyCode::Apostrophe => "Apostrophe",
        VirtualKeyCode::Apps => "Apps",
        VirtualKeyCode::Asterisk => "Asterisk",
        VirtualKeyCode::At => "At",
        VirtualKeyCode::Ax => "Ax",
        VirtualKeyCode::Backslash => "Backslash",
        VirtualKeyCode::Calculator => "Calculator",
        VirtualKeyCode::Capital => "Capital",
        VirtualKeyCode::Colon => "Colon",
        VirtualKeyCode::Comma => "Comma",
        VirtualKeyCode::Convert => "Convert",
        VirtualKeyCode::Equals => "Equals",
        VirtualKeyCode::Grave => "Grave",
        VirtualKeyCode::Kana => "Kana",
        VirtualKeyCode::Kanji => "Kanji",
        VirtualKeyCode::LAlt => "LAlt",
        VirtualKeyCode::LBracket => "LBracket",
        VirtualKeyCode::LControl => "LControl",
        VirtualKeyCode::LShift => "LShift",
        VirtualKeyCode::LWin => "LWin",
        VirtualKeyCode::Mail => "Mail",
        VirtualKeyCode::MediaSelect => "MediaSelect",
        VirtualKeyCode::MediaStop => "MediaStop",
        VirtualKeyCode::Minus => "Minus",
        VirtualKeyCode::Mute => "Mute",
        VirtualKeyCode::MyComputer => "MyComputer",
        VirtualKeyCode::NavigateForward => "NavigateForward",
        VirtualKeyCode::NavigateBackward => "NavigateBackward",
        VirtualKeyCode::NextTrack => "NextTrack",
        VirtualKeyCode::NoConvert => "NoConvert",
        VirtualKeyCode::OEM102 => "OEM102",
        VirtualKeyCode::Period => "Period",
        VirtualKeyCode::PlayPause => "PlayPause",
        VirtualKeyCode::Plus => "Plus",
        VirtualKeyCode::Power => "Power",
        VirtualKeyCode::PrevTrack => "PrevTrack",
        VirtualKeyCode::RAlt => "RAlt",
        VirtualKeyCode::RBracket => "RBracket",
        VirtualKeyCode::RControl => "RControl",
        VirtualKeyCode::RShift => "RShift",
        VirtualKeyCode::RWin => "RWin",
        VirtualKeyCode::Semicolon => "Semicolon",
        VirtualKeyCode::Slash => "Slash",
        VirtualKeyCode::Sleep => "Sleep",
        VirtualKeyCode::Stop => "Stop",
        VirtualKeyCode::Sysrq => "Sysrq",
        VirtualKeyCode::Tab => "Tab",
        VirtualKeyCode::Underline => "Underline",
        VirtualKeyCode::Unlabeled => "Unlabeled",
        VirtualKeyCode::VolumeDown => "VolumeDown",
        VirtualKeyCode::VolumeUp => "VolumeUp",
        VirtualKeyCode::Wake => "Wake",
        VirtualKeyCode::WebBack => "WebBack",
        VirtualKeyCode::WebFavorites => "WebFavorites",
        VirtualKeyCode::WebForward => "WebForward",
        VirtualKeyCode::WebHome => "WebHome",
        VirtualKeyCode::WebRefresh => "WebRefresh",
        VirtualKeyCode::WebSearch => "WebSearch",
        VirtualKeyCode::WebStop => "WebStop",
        VirtualKeyCode::Yen => "Yen",
        VirtualKeyCode::Copy => "Copy",
        VirtualKeyCode::Paste => "Paste",
        VirtualKeyCode::Cut => "Cut",
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KeyDown {
    pub key: &'static str,
}

impl KeyDown {
    pub fn from_key(key: VirtualKeyCode) -> Self {
        Self {
            key: keycode_to_str(key),
        }
    }
}

impl_event_type_lua_api!(KeyDown);

impl UserData for KeyDown {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Index, |_lua, this, value: String| {
            Ok(match value.as_str() {
                "key" => this.key,
                _ => {
                    return Err(format!(
                        "property '{}' is not exists on the '{}'",
                        value,
                        type_name::<Self>()
                    )
                    .to_lua_err())
                }
            })
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KeyUp {
    pub key: &'static str,
}

impl KeyUp {
    pub fn from_key(key: VirtualKeyCode) -> Self {
        Self {
            key: keycode_to_str(key),
        }
    }
}

impl_event_type_lua_api!(KeyUp);

impl UserData for KeyUp {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Index, |_lua, this, value: String| {
            Ok(match value.as_str() {
                "key" => this.key,
                _ => {
                    return Err(format!(
                        "property '{}' is not exists on the '{}'",
                        value,
                        type_name::<Self>()
                    )
                    .to_lua_err())
                }
            })
        });
    }
}
