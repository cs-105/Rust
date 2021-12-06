pub mod graphics {
    extern crate sdl2;
    use sdl2::pixels::Color;
    use sdl2::rect::Rect;
    use sdl2::render::Texture;
    use sdl2::render::TextureCreator;
    use sdl2::render::TextureQuery;
    use sdl2::render::WindowCanvas;
    use sdl2::ttf::Font;

    pub fn render_text<T>(
        text: String,
        font: &Font,
        texture_creator: &TextureCreator<T>,
    ) -> Result<Texture, String> {
        let surface = font
            .render(&text)
            .blended(Color::RGBA(255, 0, 0, 255))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        Ok(texture)
    }
}
