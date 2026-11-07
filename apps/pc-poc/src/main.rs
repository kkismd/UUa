use std::error::Error;

use uua_pc_poc::{platform::Platform, render::Framebuffer};

fn main() -> Result<(), Box<dyn Error>> {
    let mut platform = Platform::new("UUa PC PoC")?;
    let mut framebuffer = Framebuffer::new();
    framebuffer.draw_test_pattern();

    while platform.is_open() {
        let _input = platform.input();
        platform.present(&framebuffer)?;
    }

    Ok(())
}
