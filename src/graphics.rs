pub mod graphics {
    extern crate sdl2;
    use sdl2::image::LoadTexture;
    use sdl2::render::{Texture, TextureCreator, WindowCanvas};
    use sdl2::video::WindowContext;
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::result::Result;
    pub struct Graphics<'l> {
        pub canvas: WindowCanvas,
        pub textures: HashMap<String, Rc<Texture<'l>>>,
        _texture_creator: TextureCreator<WindowContext>,
    }
    impl<'l> Graphics<'l> {
        pub fn load_texture(&'l mut self, path: &str) -> Result<Rc<Texture<'l>>, String> {
            self.textures.get(path).cloned().map_or_else(
                || {
                    let text = self._texture_creator.load_texture(path)?;
                    let resource = Rc::new(text);
                    self.textures.insert(path.to_string(), resource.clone());
                    Ok(resource)
                },
                Ok,
            )
        }
    }
}
