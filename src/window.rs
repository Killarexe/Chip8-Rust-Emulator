use sdl2::{Sdl, render::Canvas, EventPump, video::Window, VideoSubsystem, pixels::Color, event::Event};

pub struct SdlWindow {
    title: &'static str,
    width: u32,
    height: u32,
    sdl_context: Sdl,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    video_subsystem: VideoSubsystem,
    clear_color: Color,
    needs_quit: bool
}

impl SdlWindow {
    pub fn new(title: &'static str, width: u32, height: u32, clear_color: Color) -> Self {
        let sdl_context: Sdl = sdl2::init().unwrap();
        let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();

        let window: Window = video_subsystem.window(title, width, height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();

        canvas.set_draw_color(clear_color);
        canvas.clear();
        canvas.present();

        let event_pump: EventPump = sdl_context.event_pump().unwrap();

        Self{
            title,
            width,
            height,
            sdl_context,
            canvas,
            event_pump,
            video_subsystem,
            clear_color,
            needs_quit: false
        }
    }

    pub fn update_event(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.needs_quit = true,
                _ => {}
            }
        }
    }

    pub fn prepare_render(&mut self) {
        self.canvas.clear();
    }

    pub fn render(&mut self) {
        self.canvas.present();
    }

    pub fn get_title(&self) -> &'static str {
        self.title
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn get_sdl_context(&self) -> &Sdl {
        &self.sdl_context
    }

    pub fn get_canvas(&self) -> &Canvas<Window> {
        &self.canvas
    }

    pub fn get_canvas_mut(&mut self) -> &mut Canvas<Window> {
        &mut self.canvas
    }

    pub fn get_event_pump(&mut self) -> &mut EventPump {
        &mut self.event_pump
    }

    pub fn get_video_subsystem(&self) -> &VideoSubsystem {
        &self.video_subsystem
    }

    pub fn get_clear_color(&self) -> Color {
        self.clear_color
    }

    pub fn is_window_needs_quit(&self) -> bool {
        self.needs_quit
    }

    pub fn set_title(&mut self, title: &'static str) {
        self.canvas.window_mut().set_title(title).unwrap();
        self.title = title;
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.canvas.window_mut().set_size(width, height).unwrap();
        self.width = width;
        self.height = height;
    }

    pub fn set_clear_color(&mut self, clear_color: Color) {
        self.clear_color = clear_color;
    }
}
