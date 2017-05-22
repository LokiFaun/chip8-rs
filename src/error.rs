use super::*;

#[derive(Debug)]
pub enum Chip8Error {
    IntegerOrSdlError,
    WindowBuildError,
    Message(String),
}

impl From<String> for Chip8Error {
    fn from(msg: String) -> Chip8Error {
        Chip8Error::Message(msg)
    }
}

impl From<sdl2::IntegerOrSdlError> for Chip8Error {
    fn from(_: sdl2::IntegerOrSdlError) -> Chip8Error {
        Chip8Error::IntegerOrSdlError
    }
}

impl From<sdl2::video::WindowBuildError> for Chip8Error {
    fn from(_: sdl2::video::WindowBuildError) -> Chip8Error {
        Chip8Error::WindowBuildError
    }
}

impl From<std::boxed::Box<std::any::Any + std::marker::Send>> for Chip8Error {
    fn from(err: std::boxed::Box<std::any::Any + std::marker::Send>) -> Chip8Error {
        Chip8Error::Message(format!("{:?}", err))
    }
}