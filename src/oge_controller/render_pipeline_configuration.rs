/// Descibes the method used for anti-aliasing
pub enum AntiAliasingMode {
    /// Do not use any anti-aliasing method
    None = 1,
    /// 4x MSAA
    Msaa4x = 4,
}

/// Configure the render pipeline
pub struct RenderPipelineConfiguration {
    /// The anti-aliasing mode to use
    pub anti_aliasing: AntiAliasingMode,
}

impl Default for RenderPipelineConfiguration {
    fn default() -> Self {
        Self {
            anti_aliasing: AntiAliasingMode::Msaa4x,
        }
    }
}