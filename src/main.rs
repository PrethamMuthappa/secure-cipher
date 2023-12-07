
use eframe::{egui, HardwareAcceleration, Theme};
#[allow(unused_imports)]
use egui::{Color32, FontId, Id, Layout, RichText, Sense, Vec2, Pos2, vec2, Align};

fn main() {
    let nativeoption = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        fullscreen: false,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: Option::from(Vec2::new(550f32, 350f32)),
        min_window_size: None,
        max_window_size: None,
        resizable: true,
        transparent: false,
        mouse_passthrough: false,
        active: false,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: HardwareAcceleration::Off,
        renderer: Default::default(),
        follow_system_theme: false,
        default_theme: Theme::Dark,
        run_and_return: false,
        event_loop_builder: None,
        window_builder: None,
        shader_version: None,
        centered: false,
        app_id: None,
        persist_window: false,
    };
    eframe::run_native(
        "secure-cypher",
        nativeoption,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    )
    .expect("error")
}

#[derive(Default)]
#[allow(dead_code)]
struct MyEguiApp {
   showslide:bool,
   editte:String,
   texts:String
}

#[allow(unused_variables)]
impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
       
       Self {
        showslide:false,
        editte:String::new(),
        texts:String::new(),
       };

        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::Center),|ui| {
                ui.heading(RichText::new("SECURE CYPHER").color(Color32::WHITE));
            });
            ui.add_space(20.0);
            ui.with_layout(Layout::top_down(Align::Center),|ui| {

                ui.label(RichText::new("A self hosted password manager for linux. Free and open source").color(Color32::WHITE))
            });

            ui.add_space(20.10);

           ui.with_layout(Layout::top_down(Align::Center), |ui| {
 
            ui.text_edit_multiline(& mut self.editte);
            ui.add_space(10.0);
            if ui.button("cick to submit").clicked() {
               self.texts+=&self.editte;
               
            }

           });

         
            

            
        });

    }
}
