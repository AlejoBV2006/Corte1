use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::Aead, aes::Aes256};
use anyhow::Ok;
use eframe::egui;
use egui_extras::{Column, TableBuilder};
use std::{collections::{HashMap, HashSet}, process};

fn main() -> eframe::Result<()>{
    let native_options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 600.0]),..Default::default()
    };

    eframe::run_native("KeePass", native_options, Box::new(|_cc| Box::new(KeePassApp::default())))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct KeePassApp{
    search_text: String,
    selected_group: String,
    groups: HashSet<String>,
    tickets: usize,
    passwords: HashMap<String, HashMap<String, Vec<String>>>,
    windows_file_new: bool,
    new_path: Option<std::path::PathBuf>,
    master_key: String,
    confirmation_master_key: String
}

impl Default for KeePassApp{
    fn default() -> Self {

        let mut default_groups: HashSet<String> = HashSet::new();
        default_groups.insert("General".to_string());
        default_groups.insert("Windows".to_string());
        default_groups.insert("Red".to_string());

        let mut passwords_defaults: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
        let mut _rows_default: HashMap<String, Vec<String>> = vec![
            ("titulo".to_string(), vec!["Ejemplo1".to_string(), "Ejemplo2".to_string()]),
            ("users".to_string(), vec!["admin".to_string(), "santipro3991".to_string()]), 
            ("passwords".to_string(), vec!["123456".to_string(), "654321".to_string()]),
            ("urls".to_string(), vec!["https://phpmyadmin.com".to_string(), "https://facebook.com".to_string()]),
            ("notes".to_string(), vec!["lorem ipsu lorem ipsu".to_string(), "ipsu lorem ipsu lorem".to_string()])
            ].into_iter().collect();
        let default_ticket: usize = _rows_default["titulo"].len();

        for group in &default_groups{
            passwords_defaults.insert(group.clone(), _rows_default.clone());
        }
        Self {
            search_text: String::new(),
            selected_group: "General".to_string(),
            groups: default_groups.into_iter().collect(),
            tickets: default_ticket,
            passwords: passwords_defaults,
            windows_file_new: false,
            new_path: None,
            master_key: String::new(),
            confirmation_master_key: String::new()
        }
    }
}

impl eframe::App for KeePassApp{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        if self.windows_file_new {
                        ctx.show_viewport_immediate(egui::ViewportId::from_hash_of("file_new_windows"), egui::ViewportBuilder::default()
                            .with_title("Llave maestra")
                            .with_inner_size([300.0, 200.0]), |ctx, class| {
                                egui::CentralPanel::default()
                                    .show(ctx, |ui| {
                                            ui.label("Escribe la contraseña: ");
                                            ui.add(egui::TextEdit::singleline(&mut self.master_key)
                                                .password(true)
                                                .hint_text("Tu llave maestra"));

                                            ui.separator();
                                        
                                            ui.label("Repite la contraseña: ");
                                            ui.add(egui::TextEdit::singleline(&mut self.confirmation_master_key)
                                                .password(true)
                                                .hint_text("Tu llave maestra"));                                            

                                            if ui.button("Confirmar").clicked() {

                                                if !self.master_key.is_empty() && !self.confirmation_master_key.is_empty() && &self.master_key == &self.confirmation_master_key{
                                                    
                                                    if let Some(path) = &self.new_path{
                                                        
                                                        let json_data = serde_json::to_string(&self.passwords).unwrap();
    
                                                        let mut key_bytes = [0u8; 32];
                                                        let password_bytes = self.master_key.as_bytes();
                                                        
                                                        for i in 0..password_bytes.len().min(32) {
                                                            key_bytes[i] = password_bytes[i];
                                                        }
    
                                                        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
                                                        let cipher = Aes256Gcm::new(key);
                                                        let nonce  = Nonce::from_slice(b"unique nonce");
                                                        
                                                        match cipher.encrypt(nonce, json_data.as_ref()) {
                                                            core::result::Result::Ok(ciphertext) => {
                                                                match std::fs::write(path, ciphertext) {
                                                                    core::result::Result::Ok(_) => {
                                                                        self.windows_file_new = false;
                                                                        self.master_key.clear();
                                                                        self.confirmation_master_key.clear();
                                                                    }
                                                                    Err(e) => {
                                                                        eprintln!("{}", e);
                                                                    }
                                                                }
                                                            } 
                                                            Err(e) => {
                                                                eprintln!("{}", e);
                                                            }
                                                        }
                                                        }
                                                        
                                                    }

                                                }
                                            

                                            if ctx.input(|ui| ui.viewport().close_requested()) {
                                                self.windows_file_new = false;
                                            }
                                                                                
                                        });
                            });
        }
        
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
              ui.menu_button("Archivo", |ui| {
                if ui.button("Nueva base de datos").clicked() { 
                    let file_new = rfd::FileDialog::new()
                        .set_file_name("database.kdbx")
                        .add_filter("KeePass DB (*.kdbx)", &["kdbx", "db"])
                        .save_file();
                    
                    

                    if let Some(mut file) = file_new {

                        if file.extension().and_then(|ui| ui.to_str()) != Some("kdbx"){
                            file.set_extension("kdbx");
                        }

                        self.new_path = Some(file);
                        self.windows_file_new = true;
                    }

                    

                 }
                if ui.button("Abrir base de datos").clicked() {                     
                    let files_open = rfd::FileDialog::new()
                        .set_directory("C:\\Users\\santi")
                        .add_filter("KeePass DB (*kdbx)", &["kdbx", "db"])
                        .pick_file();

                    if let Some(file_open) = files_open {
                        println!("good");
                    } 
                ui.close_menu();
                }
                ui.separator();
                if ui.button("Salir").clicked() {process::exit(0);}
              });  

              ui.menu_button("Group", |ui| {
                if ui.button("Nose").clicked() { /* code */ }
              });
            }); 
        });

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| { 
            ui.horizontal(|ui| {
                if ui.button("➕").on_hover_text("Nueva entrada").clicked() {  }
                if ui.button("💾").on_hover_text("Guardar").clicked() {  } 
                ui.separator();
                ui.label("Buscar:");
                ui.text_edit_singleline(&mut self.search_text);                    
            });
        });

        egui::SidePanel::left("tree_panel")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Grupos");
                ui.separator();

                for group in &self.groups{
                    if ui.selectable_label(self.selected_group == *group, group).clicked() {
                        self.selected_group = group.clone();
                    }
                }
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading(format!("Entradas del grupo {}", self.selected_group));
                ui.separator();

                let table = TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::auto().at_least(100.0))
                    .column(Column::auto().at_least(100.0))
                    .column(Column::auto().at_least(100.0))
                    .column(Column::auto().at_least(100.0))
                    .column(Column::remainder());

                table.header(20.0, |mut header| {
                    header.col(|ui| {ui.strong("Titulo");});
                    header.col(|ui| {ui.strong("Usuario");});
                    header.col(|ui| {ui.strong("Contraseña");});
                    header.col(|ui| {ui.strong("URL");});
                    header.col(|ui| {ui.strong("Notas");});
                })
                .body(|mut body| {
                    if let Some(group_data) = self.passwords.get(&self.selected_group){
                        for i in 0..self.tickets{

                            body.row(25.0, |mut row| {
                                row.col(|ui| {
                                    let title = group_data
                                        .get("titulo")
                                        .and_then(|v| v.get(i))
                                        .map(|s| s.as_str())
                                        .unwrap_or("");
                                    ui.label(title);
                                });

                                row.col(|ui|{
                                    let user = group_data
                                        .get("users")
                                        .and_then(|v| v.get(i))
                                        .map(|s| s.as_str())
                                        .unwrap_or("");
                                    ui.label(user);
                                });

                                row.col(|ui| {
                                    let password = group_data
                                        .get("passwords")
                                        .and_then(|v| v.get(i))
                                        .map(|s| s.as_str())
                                        .unwrap_or("");
                                    ui.label(password);                                    
                                });

                                row.col(|ui| {
                                    let url = group_data
                                        .get("urls")
                                        .and_then(|v| v.get(i))
                                        .map(|s| s.as_str())
                                        .unwrap_or("");
                                    ui.label(url);
                                });

                                row.col(|ui| {
                                    let note = group_data
                                        .get("notes")
                                        .and_then(|v| v.get(i))
                                        .map(|s| s.as_str())
                                        .unwrap_or("");
                                    ui.label(note);
                                });
                            
                            });
                        }    
                    }
                });
            });
    }
}