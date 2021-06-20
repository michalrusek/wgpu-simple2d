pub struct State {

}

impl State {
    pub async fn new(window: &winit::window::Window) -> Self {
        Self {}
    }

    pub fn update_and_render(&mut self) {
        self.update();
        self.render();
    }

    fn update(&mut self) {
        // WORLD LOGIC RUN
        // TODO: MAKE EXTENSIBLE THROUGH SOME SCRIPTS MAYBE? LIKE INSERT A V8 AND RUN JS SCRIPTS FOR THE GAME LOGIC?
    }
    fn render(&mut self) {
        // SEND BUFFERS AND SHIT TO GPU AND RENDER
    }
    pub fn handle_window_event(&mut self, event: &winit::event::WindowEvent) -> bool {
        false
    }
    pub fn resize(&mut self, physical_size: winit::dpi::PhysicalSize<u32>) {}
}