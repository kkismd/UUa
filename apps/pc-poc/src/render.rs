pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 240;
pub const PIXEL_COUNT: usize = WIDTH * HEIGHT;

const BLACK: u32 = 0x000000;
const RED: u32 = 0xCC3333;
const GREEN: u32 = 0x33AA66;
const BLUE: u32 = 0x3366CC;
const WHITE: u32 = 0xEEEEEE;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Framebuffer {
    pixels: Vec<u32>,
}

impl Default for Framebuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Framebuffer {
    pub fn new() -> Self {
        Self {
            pixels: vec![BLACK; PIXEL_COUNT],
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.pixels.fill(color & 0x00FF_FFFF);
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) -> bool {
        let Some(index) = Self::index(x, y) else {
            return false;
        };
        self.pixels[index] = color & 0x00FF_FFFF;
        true
    }

    pub fn pixel(&self, x: usize, y: usize) -> Option<u32> {
        Self::index(x, y).map(|index| self.pixels[index])
    }

    pub fn pixels(&self) -> &[u32] {
        &self.pixels
    }

    pub fn draw_test_pattern(&mut self) {
        self.clear(BLACK);
        let third_width = WIDTH / 3;
        for y in 0..HEIGHT {
            let colors = if y < HEIGHT / 2 {
                [RED, GREEN, BLUE]
            } else {
                [BLUE, RED, GREEN]
            };
            for (section, color) in colors.into_iter().enumerate() {
                let start = section * third_width;
                let end = if section == 2 {
                    WIDTH
                } else {
                    start + third_width
                };
                for x in start..end {
                    self.set_pixel(x, y, color);
                }
            }
        }
        for y in 100..140 {
            for x in 140..180 {
                self.set_pixel(x, y, WHITE);
            }
        }
    }

    fn index(x: usize, y: usize) -> Option<usize> {
        (x < WIDTH && y < HEIGHT).then_some(y * WIDTH + x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_has_logical_dimensions_and_initial_color() {
        let buffer = Framebuffer::new();
        assert_eq!(buffer.pixels().len(), 320 * 240);
        assert_eq!(buffer.pixel(0, 0), Some(BLACK));
    }

    #[test]
    fn clear_and_pixel_access_update_the_expected_pixel() {
        let mut buffer = Framebuffer::new();
        buffer.clear(0xFF12_3456);
        assert!(buffer.set_pixel(WIDTH - 1, HEIGHT - 1, 0xFFAB_CDEF));
        assert_eq!(buffer.pixel(0, 0), Some(0x123456));
        assert_eq!(buffer.pixel(WIDTH - 1, HEIGHT - 1), Some(0xABCDEF));
    }

    #[test]
    fn out_of_range_access_is_safe() {
        let mut buffer = Framebuffer::new();
        assert!(!buffer.set_pixel(WIDTH, 0, WHITE));
        assert!(!buffer.set_pixel(0, HEIGHT, WHITE));
        assert_eq!(buffer.pixel(WIDTH, 0), None);
        assert_eq!(buffer.pixel(0, HEIGHT), None);
        assert_eq!(buffer.pixels().len(), PIXEL_COUNT);
    }

    #[test]
    fn test_pattern_has_distinct_regions() {
        let mut buffer = Framebuffer::new();
        buffer.draw_test_pattern();
        assert_eq!(buffer.pixel(0, 0), Some(RED));
        assert_eq!(buffer.pixel(110, 10), Some(GREEN));
        assert_eq!(buffer.pixel(220, 10), Some(BLUE));
        assert_eq!(buffer.pixel(150, 120), Some(WHITE));
        assert_eq!(buffer.pixel(150, 90), Some(GREEN));
    }
}
