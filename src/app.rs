use crate::main_old::{fib, FibContents, FibList, find_fib_series, find_sum_of_fib, LIMIT, string_from_fib_list};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {


    #[serde(skip)]
    fib_list: Option<FibList>,
    #[serde(skip)]
    sum_series: Option<Vec<FibContents>>,
    #[serde(skip)]
    sum_series_number: i32,
    #[serde(skip)]
    fib_search_number: i32,

}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {

            fib_list: Option::from({
                fib(1000, LIMIT)
            }),


            sum_series: None,
            sum_series_number: 0,
            fib_search_number: 0,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { .. } = self;

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        }); // windows only top panel

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Input Panel");
            ui.style_mut().spacing.slider_width = 100.0;
            ui.add(egui::Slider::new(&mut self.fib_search_number,1..=999_999).text("Fibonacci search number").smart_aim(false));

            ui.add(egui::Slider::new(&mut self.sum_series_number, 0..=999_999).text("Sum series number").smart_aim(false));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.fib_list.as_ref() {
                Some(list) => {
                    let fib_list_text = format!("Fibonacci list: {}",string_from_fib_list(list));
                    ui.label(fib_list_text); // display the fibonacci list to the user

                    match find_fib_series(self.fib_search_number as FibContents, list) {
                        Ok(found_index) => {
                            ui.label(format!("Number searched for found at location: {}", found_index));
                        }
                        Err(err) => {
                            ui.label(format!("{}", err));
                        }
                    }

                    match find_sum_of_fib(self.sum_series_number as FibContents, list) {
                        Ok(sum) => {
                            ui.label(format!("Sum Series: {:?}", sum));
                        }
                        Err(err) => {
                            ui.label(format!("Unable to make up sum, error: {}", err));
                        }
                    }
                }
                None => {}
            }



            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
