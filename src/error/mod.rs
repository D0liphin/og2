use std::{fmt, path::PathBuf};
macro_rules! impl_display {
    ($Struct:ty) => {
        impl std::fmt::Display for $Struct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.description)
            }
        }
    };
}

#[derive(Debug)]
pub enum Error {
    Texture(TextureError),
    Render(RenderError),
}

pub struct TextureError {
    description: String,
}

impl fmt::Debug for TextureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl_display!(TextureError);

impl TextureError {
    pub(crate) fn open(path_buf: &PathBuf) -> Error {
        Error::Texture(TextureError {
            description: format!("could not open image {:?}", path_buf),
        })
    }

    pub(crate) fn decode(path_buf: &PathBuf) -> Error {
        Error::Texture(TextureError {
            description: format!("could not decode image {:?}", path_buf),
        })
    }

    pub(crate) fn load_bytes() -> Error {
        Error::Texture(TextureError {
            description: "could not load image from bytes".to_owned(),
        })
    }

    pub(crate) fn format(label: Option<&str>) -> Error {
        Error::Texture(TextureError {
            description: format!("image {:?} is not of the correct format", label),
        })
    }
}

/// Returned if there was an error when rendering.
pub struct RenderError {
    description: String,
}

impl_display!(RenderError);

impl fmt::Debug for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}


impl RenderError {
    pub(crate) fn frame() -> Error {
        Error::Render(RenderError {
            description: format!("could not get current frame")
        })
    }
}

pub type Result<T> = std::result::Result<T, Error>;