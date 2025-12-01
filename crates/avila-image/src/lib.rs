// AvilaImage - Native Image Processing
// Zero External Dependencies 🦀

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>, // RGB pixels
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![0; (width * height * 3) as usize],
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> (u8, u8, u8) {
        let index = ((y * self.width + x) * 3) as usize;
        (self.data[index], self.data[index + 1], self.data[index + 2])
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) {
        let index = ((y * self.width + x) * 3) as usize;
        self.data[index] = r;
        self.data[index + 1] = g;
        self.data[index + 2] = b;
    }

    pub fn resize(&self, new_width: u32, new_height: u32) -> Image {
        let mut result = Image::new(new_width, new_height);
        
        for y in 0..new_height {
            for x in 0..new_width {
                let src_x = (x * self.width) / new_width;
                let src_y = (y * self.height) / new_height;
                let (r, g, b) = self.get_pixel(src_x, src_y);
                result.set_pixel(x, y, r, g, b);
            }
        }
        
        result
    }

    pub fn to_grayscale(&self) -> Image {
        let mut result = Image::new(self.width, self.height);
        
        for y in 0..self.height {
            for x in 0..self.width {
                let (r, g, b) = self.get_pixel(x, y);
                let gray = ((r as u32 + g as u32 + b as u32) / 3) as u8;
                result.set_pixel(x, y, gray, gray, gray);
            }
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_creation() {
        let img = Image::new(100, 100);
        assert_eq!(img.width, 100);
        assert_eq!(img.height, 100);
    }

    #[test]
    fn test_pixel_operations() {
        let mut img = Image::new(10, 10);
        img.set_pixel(5, 5, 255, 0, 0);
        let (r, g, b) = img.get_pixel(5, 5);
        assert_eq!((r, g, b), (255, 0, 0));
    }

    #[test]
    fn test_resize() {
        let img = Image::new(100, 100);
        let resized = img.resize(50, 50);
        assert_eq!(resized.width, 50);
        assert_eq!(resized.height, 50);
    }
}
