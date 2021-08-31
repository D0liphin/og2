use std::{fmt, path::PathBuf};
macro_rules! impl_display {
    ($Struct:ty) => {
        impl std::fmt::Display for $Struct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.info)
            }
        }
    };
}

#[derive(Debug)]
pub enum OgeError {
    Texture(TextureError),
    Render(RenderError),
}

pub struct TextureError {
    info: String,
}

impl fmt::Debug for TextureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.info)
    }
}

impl_display!(TextureError);

impl TextureError {
    pub(crate) fn open(path_buf: &PathBuf) -> OgeError {
        OgeError::Texture(TextureError {
            info: format!("Could not open image {:?}", path_buf),
        })
    }

    pub(crate) fn decode(path_buf: &PathBuf) -> OgeError {
        OgeError::Texture(TextureError {
            info: format!("Could not decode image {:?}", path_buf),
        })
    }

    pub(crate) fn load_bytes() -> OgeError {
        OgeError::Texture(TextureError {
            info: "Could not load image from bytes".to_owned(),
        })
    }

    pub(crate) fn format(label: Option<&str>) -> OgeError {
        OgeError::Texture(TextureError {
            info: format!("Image {:?} is not of the correct format", label),
        })
    }
}

#[derive(Debug)]
pub struct RenderError {}
