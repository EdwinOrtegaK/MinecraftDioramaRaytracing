pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}

impl Framebuffer {
    // Constructor del framebuffer
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    // Método para hacer upscale bilineal
    pub fn upscale_bilineal(&self, new_width: usize, new_height: usize) -> Framebuffer {
        let mut high_res_buffer = vec![0; new_width * new_height];

        for y in 0..new_height {
            let src_y = y as f32 * (self.height as f32 / new_height as f32);
            let src_y1 = src_y.floor() as usize;
            let src_y2 = (src_y1 + 1).min(self.height - 1);
            let weight_y = src_y - src_y1 as f32;

            for x in 0..new_width {
                let src_x = x as f32 * (self.width as f32 / new_width as f32);
                let src_x1 = src_x.floor() as usize;
                let src_x2 = (src_x1 + 1).min(self.width - 1);
                let weight_x = src_x - src_x1 as f32;

                let c11 = self.buffer[src_y1 * self.width + src_x1];
                let c12 = self.buffer[src_y1 * self.width + src_x2];
                let c21 = self.buffer[src_y2 * self.width + src_x1];
                let c22 = self.buffer[src_y2 * self.width + src_x2];

                let color = Self::bilinear_interpolation(c11, c12, c21, c22, weight_x, weight_y);
                high_res_buffer[y * new_width + x] = color;
            }
        }

        Framebuffer {
            width: new_width,
            height: new_height,
            buffer: high_res_buffer,
        }
    }

    // Interpolación bilineal para los colores
    fn bilinear_interpolation(c11: u32, c12: u32, c21: u32, c22: u32, weight_x: f32, weight_y: f32) -> u32 {
        let r11 = ((c11 >> 16) & 0xFF) as f32;
        let g11 = ((c11 >> 8) & 0xFF) as f32;
        let b11 = (c11 & 0xFF) as f32;

        let r12 = ((c12 >> 16) & 0xFF) as f32;
        let g12 = ((c12 >> 8) & 0xFF) as f32;
        let b12 = (c12 & 0xFF) as f32;

        let r21 = ((c21 >> 16) & 0xFF) as f32;
        let g21 = ((c21 >> 8) & 0xFF) as f32;
        let b21 = (c21 & 0xFF) as f32;

        let r22 = ((c22 >> 16) & 0xFF) as f32;
        let g22 = ((c22 >> 8) & 0xFF) as f32;
        let b22 = (c22 & 0xFF) as f32;

        let r = (r11 * (1.0 - weight_x) * (1.0 - weight_y)
            + r12 * weight_x * (1.0 - weight_y)
            + r21 * (1.0 - weight_x) * weight_y
            + r22 * weight_x * weight_y) as u32;

        let g = (g11 * (1.0 - weight_x) * (1.0 - weight_y)
            + g12 * weight_x * (1.0 - weight_y)
            + g21 * (1.0 - weight_x) * weight_y
            + g22 * weight_x * weight_y) as u32;

        let b = (b11 * (1.0 - weight_x) * (1.0 - weight_y)
            + b12 * weight_x * (1.0 - weight_y)
            + b21 * (1.0 - weight_x) * weight_y
            + b22 * weight_x * weight_y) as u32;

        (r << 16) | (g << 8) | b
    }

    // Obtener el buffer
    pub fn get_buffer(&self) -> &[u32] {
        &self.buffer
    }
}