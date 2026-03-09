use eframe::egui;
use chrono::{Datelike, Local, NaiveDate};

fn main() -> eframe::Result<()>{
    let native_options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0])
            .with_max_inner_size([900.0, 600.0])
            .with_min_inner_size([900.0, 600.0])
            .with_resizable(false)
            .with_maximize_button(false),
            ..Default::default()
    };

    eframe::run_native("Parcial", native_options, Box::new(|cc| {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Box::new(ProgramApp::default())}))
}

struct ProgramApp {
    colors: Vec<String>,
    selected_color: String,
    animals: Vec<String>,
    selected_animals: String,
    current_screen: Pantalla,
    games: Vec<(String, bool)>,
    selected_games: Vec<String>,
    start_date_1: NaiveDate,
    start_date_2: NaiveDate,
    end_date_1: NaiveDate,
    end_date_2: NaiveDate,
    days_1: i64,
    days_2: i64,
    uploaded_image: Option<String>
}

#[derive(PartialEq, Default)]
enum Pantalla{
    #[default]
    RadioButton,
    ElapsedTime,
    ElapsedTime2,
    VideoGames,
    Images

}

impl Default for ProgramApp{
    fn default() -> Self {

        let colors: Vec<String> = vec![
                "Marron".to_string(), 
                "Amarillo".to_string(),
                "Verde".to_string(),
                "Violeta".to_string()
        ];

        let animals: Vec<String> = vec![
                "Ballena".to_string(), 
                "Delfin".to_string(),
                "Tiburon".to_string(),
                "Barracuda".to_string(),
                "Salmon".to_string(),
        ];

        let mut games: Vec<(String, bool)> =Vec::new(); 
        match std::fs::read_to_string("C:/Users/santi/Desktop/Programas/Rust/prueba2/src/categorias.txt") {
            Ok(content) => {
                for category in content.lines(){
                    games.push((category.to_string(), false));
                }
            }Err(e) =>{
                println!("{}", e);
            }
        }
            

        Self {
            selected_color: colors[0].clone(),
            colors,
            selected_animals: animals[0].clone(),
            animals,
            current_screen: Pantalla::default(),
            games: games,
            selected_games: Vec::new(),
            start_date_1: NaiveDate::from_ymd_opt(1972, 6, 26).unwrap(),
            start_date_2: NaiveDate::from_ymd_opt(1972, 6, 26).unwrap(),
            end_date_1: Local::now().date_naive(),
            end_date_2: Local::now().date_naive(),
            days_1: 0,
            days_2: 0,
            uploaded_image: None
        }
    }
}

impl eframe::App for ProgramApp{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {

            ui.horizontal(|ui| {
                if ui.button("RadioButton").clicked() {
                    self.current_screen = Pantalla::RadioButton;
                }

                if ui.button("Tiempo transcurrido").clicked() {
                    self.current_screen = Pantalla::ElapsedTime;
                }

                if ui.button("Tiempo transcurrido 2").clicked() {
                    self.current_screen = Pantalla::ElapsedTime2
                }

                if ui.button("Videojuegos").clicked() {
                    self.current_screen = Pantalla::VideoGames
                }

                if ui.button("Imagenes").clicked() {
                    self.current_screen = Pantalla::Images;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui|{
            match self.current_screen{
                Pantalla::RadioButton => {
                    
                    ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui|{
                        ui.vertical(|ui|{
                            ui.add_space(60.0);

                            ui.columns(2, |columns| {
    
                                columns[0].vertical_centered(|ui| {
                                        
                                    ui.label(egui::RichText::new("Colores").strong());
                                    ui.add_space(10.0);


                                    egui::Grid::new("grid_colores")
                                        .num_columns(1)
                                        .spacing([10.0, 8.0])
                                        .show(ui, |ui|{
                                            for color in &self.colors{
                                                ui.radio_value(&mut self.selected_color, color.clone(), color);
                                                ui.end_row();
                                                }
                                        });

                                        ui.add_space(36.2);
                                        ui.separator();
                                        ui.heading(format!("Elegido: {}", self.selected_color));
            
                                    });
                                    
                                columns[1].vertical_centered(|ui| {
                                        ui.label(egui::RichText::new("Animales").strong());
                                        ui.add_space(10.0);

                                        egui::Grid::new("grid_animales")
                                            .num_columns(1)
                                            .spacing([10.0, 8.0])
                                            .show(ui, |ui| {
                                                for animal in &self.animals {
                                                        ui.radio_value(&mut self.selected_animals, animal.clone(), animal);
                                                        ui.end_row();
                                                        }
                                                });

                                        ui.add_space(10.0);
                                        ui.separator();
                                        ui.heading(format!("Elegido: {}", self.selected_animals));
                                    });
    
                        });

                        });
                    });
                }

                Pantalla::ElapsedTime => {
                    ui.vertical_centered(|ui| {
                        ui.add_space(100.0);

                        let width_contenido = 400.0; 

                        ui.allocate_ui(egui::vec2(width_contenido, 200.0), |ui| {
                            egui::Grid::new("grid_date1_center")
                                .num_columns(2)
                                .spacing([15.0, 20.0])
                                .min_col_width(100.0) 
                                .show(ui, |ui| {
                                    
                                    // Fila 1
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.label(egui::RichText::new("Fecha inicial:").strong());
                                    });
                                    ui.horizontal(|ui| {
                                        ui.add(egui_extras::DatePickerButton::new(&mut self.start_date_1).id_source("sd1"));
                                        ui.add_space(5.0);
                                        ui.label(format!("{}, {} de {} de {}", 
                                            name_day(self.start_date_1), self.start_date_1.day(), name_month(self.start_date_1), self.start_date_1.year()));
                                    });
                                    ui.end_row();

                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.label(egui::RichText::new("Fecha final:").strong());
                                    });
                                    ui.horizontal(|ui| {
                                        ui.add(egui_extras::DatePickerButton::new(&mut self.end_date_1).id_source("ed1"));
                                        ui.add_space(5.0);
                                        ui.label(format!("{}, {} de {} de {}", 
                                            name_day(self.end_date_1), self.end_date_1.day(), name_month(self.end_date_1), self.end_date_1.year()));
                                    });
                                    ui.end_row();
                                });
                        });

                        ui.add_space(30.0);

                        if ui.add(egui::Button::new("Calcular diferencia").min_size(egui::vec2(220.0, 35.0))).clicked() {
                            let duration = self.end_date_1.signed_duration_since(self.start_date_1);
                            self.days_1 = duration.num_days();
                        }

                        ui.add_space(20.0);
                        
                        ui.label(egui::RichText::new(format!("Dias {}", self.days_1))
                            .size(24.0)
                            .strong()
                            .color(egui::Color32::LIGHT_BLUE));
                    });
                }
                Pantalla::ElapsedTime2 => {
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);

                        egui::Grid::new("grid_central_calendarios")
                            .num_columns(3)
                            .spacing([30.0, 0.0]) 
                            .show(ui, |ui| {
                                
                                let total_width = ui.available_width();
                                let calendar_width = 240.0; 
                                let gap = 30.0;
                                let margin = (total_width - (calendar_width * 2.0 + gap)) / 2.0;
                                
                                ui.add_space(margin.max(0.0));

                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.label(egui::RichText::new("Fecha Inicial").strong());
                                        ui.add_space(8.0);
                                        dibujar_calendario_fijo(ui, &mut self.start_date_2, "cal_1");
                                    });

                                    ui.add_space(gap);

                                    ui.vertical(|ui| {
                                        ui.label(egui::RichText::new("Fecha Final").strong());
                                        ui.add_space(8.0);
                                        dibujar_calendario_fijo(ui, &mut self.end_date_2, "cal_2");
                                    });
                                });

                                ui.end_row();
                            });

                        ui.add_space(40.0);

                        if ui.add(egui::Button::new("Calcular diferencia").min_size(egui::vec2(250.0, 40.0))).clicked() {
                            self.days_2 = self.end_date_2.signed_duration_since(self.start_date_2).num_days();
                        }

                        ui.add_space(20.0);
                        ui.label(egui::RichText::new(format!("Días transcurridos: {}", self.days_2))
                            .size(24.0)
                            .strong()
                            .color(egui::Color32::LIGHT_BLUE));
                    });
                }

                Pantalla::VideoGames => {

                    ui.columns(2, |columns| {

                        columns[0].vertical(|ui| {

                            ui.heading("Categorias existentes");
                            egui::ScrollArea::vertical()
                                .id_source("selected_list")
                                .max_height(400.0)
                                .show(ui, |ui|{
                                    for (name, check) in self.games.iter_mut() {
                                        ui.checkbox(check, name.as_str());
                                    }
                                })
                        });

                        columns[1].vertical(|ui| {
                            ui.heading("Categorias seleccionadas");

                            if ui.button("Filtrar").clicked() {
                                self.selected_games = self.games.iter()
                                    .filter(|(_, check)| *check)
                                    .map(|(name, _)| name.clone())
                                    .collect();
                            }

                            ui.add_space(10.0);


                            egui::ScrollArea::vertical()
                                .id_source("summary_list")
                                .show(ui, |ui| {
                                    if self.selected_games.is_empty(){
                                        ui.label(egui::RichText::new("Ninguno"));
                                    } else{
                                        for name in &self.selected_games{
                                            ui.label(format!("{}", name));
                                        }
                                    }
                                });
                        }); 

                    });

                }

                Pantalla::Images => {

                    ui.vertical_centered(|ui| {

                        if ui.button("Cargar Imagen").clicked() {

                            if let Some(path) = rfd::FileDialog::new().pick_file(){



                                self.uploaded_image = Some(format!("file://{}", path.to_string_lossy().replace("\\", "/")));

                            }

                        }

                    });



                    ui.add_space(20.0);



                    if let Some(url) = &self.uploaded_image {

                        ui.add(

                            egui::Image::new(url)

                                .max_width(400.0)

                                .rounding(10.0)

                        );

                    }

                }
            }
        });


    }
}

fn name_month(date: chrono::NaiveDate) -> &'static str {
    match date.month() {
        1 => "Enero",
        2 => "Febrero",
        3 => "Marzo",
        4 => "Abril",
        5 => "Mayo",
        6 => "Junio",
        7 => "Julio",
        8 => "Agosto",
        9 => "Septiembre",
        10 => "Octubre",
        11 => "Noviembre",
        12 => "Diciembre",
        _ => "Desconocido",
    }
}

fn name_day(date: chrono::NaiveDate) -> &'static str {
    match date.weekday() {
        chrono::Weekday::Mon => "Lunes",
        chrono::Weekday::Tue => "Martes",
        chrono::Weekday::Wed => "Miércoles",
        chrono::Weekday::Thu => "Jueves",
        chrono::Weekday::Fri => "Viernes",
        chrono::Weekday::Sat => "Sábado",
        chrono::Weekday::Sun => "Domingo",
    }
}

fn dibujar_calendario_fijo(ui: &mut egui::Ui, fecha: &mut chrono::NaiveDate, id: &str) {
    ui.vertical(|ui| {
        // Cabecera: Mes y Año
        ui.horizontal(|ui| {
            if ui.button("<").clicked() { 
                    *fecha = fecha.checked_sub_months(chrono::Months::new(1)).unwrap(); 
                }

    // Creamos un área de ancho fijo para el texto
            ui.allocate_ui(egui::vec2(120.0, 20.0), |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(format!("{} {}", name_month(*fecha), fecha.year()));
                });
            });

    // Botón Derecho (ahora nunca se moverá de su posición)
            if ui.button(">").clicked() { 
                *fecha = fecha.checked_add_months(chrono::Months::new(1)).unwrap(); 
            }
        });

        // Cuadrícula de días
        egui::Grid::new(id).show(ui, |ui| {
            let dias = ["dom", "lun", "mar", "mié", "jue", "vie", "sáb"];
            for dia in dias { ui.label(dia); }
            ui.end_row();

            // Lógica para rellenar los días del mes...
            let primer_dia = fecha.with_day(1).unwrap();
            let mut d = primer_dia.weekday().num_days_from_sunday();
            
            // Espacios en blanco al inicio
            for _ in 0..d { ui.label(" "); }

            let dias_del_mes = chrono::NaiveDate::from_ymd_opt(
                if fecha.month() == 12 { fecha.year() + 1 } else { fecha.year() },
                if fecha.month() == 12 { 1 } else { fecha.month() + 1 },
                1
            ).unwrap().signed_duration_since(primer_dia).num_days();

            for dia_num in 1..=dias_del_mes {
                if ui.selectable_label(fecha.day() == dia_num as u32, dia_num.to_string()).clicked() {
                    *fecha = fecha.with_day(dia_num as u32).unwrap();
                }
                d += 1;
                if d % 7 == 0 { ui.end_row(); }
            }
        });
    });
}