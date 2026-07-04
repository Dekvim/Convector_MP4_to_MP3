#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![expect(rustdoc::missing_crate_level_docs)]

use eframe::egui;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;

fn main() -> eframe::Result {
    let icon_data = load_icon();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([460.0, 360.0])
            .with_resizable(false)
            .with_maximize_button(false)
            .with_icon(Arc::new(icon_data)),
        ..Default::default()
    };
    eframe::run_native(
        "Конвертер mp4 to mp3",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

fn load_icon() -> egui::IconData {
    // include_bytes! на этапе компиляции читает файл и вставляет 
    // его содержимое прямо в бинарник. Файл на диске больше не нужен!
    let icon_bytes = include_bytes!("../assets/icon.png");
    
    let img = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .into_rgba8();
    
    let (width, height) = img.dimensions();
    let rgba = img.into_raw();

    egui::IconData { rgba, width, height }
}

#[derive(Default)]
struct MyApp {
    path_files: Vec<String>,
    is_name: bool,
    name_music: Vec<String>,
    is_convector: bool,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.set_visuals(egui::Visuals::dark());
        

        let dropped = ui.input(|i| i.raw.dropped_files.clone());

        for file in dropped {
            if let Some(path) = file.path {
                self.path_files.push(path.to_string_lossy().to_string());
                // Инициализируем пустое имя СРАЗУ при добавлении файла
                self.name_music.push(String::new()); 
                println!("{:?}", self.path_files);
            }
        }

        egui::CentralPanel::default().show(ui, |ui| {
            if !self.is_convector {
                ui.label(egui::RichText::new("Список файлов:").strong());
                if self.path_files.is_empty() {
                    ui.label(egui::RichText::new("Пока пусто...").color(egui::Color32::GRAY));
                } else {
                    egui::ScrollArea::vertical()
                        .max_height(120.0)
                        .show(ui, |ui| {
                            for (i, file) in self.path_files.iter().enumerate() {
                                let display_num = i + 1;
                                // УБРАЛИ ОТСЮДА self.name_music.push(...)

                                if self.is_name {
                                    ui.horizontal(|ui| {
                                        ui.label(format!("{display_num}. {file}"));
                                        if i < self.name_music.len() {
                                            ui.add(
                                                egui::TextEdit::singleline(&mut self.name_music[i])
                                                    .hint_text("Введите имя трека"),
                                            );
                                        }
                                    });
                                } else {
                                    ui.label(format!("{display_num}. {file}"));
                                }
                            }
                        });
                    ui.horizontal(|ui| {
                        if ui.button("Очистить список").clicked() {
                            self.path_files.clear();
                            self.name_music.clear();
                        }
                        ui.checkbox(&mut self.is_name, "Имя музыки");
                    });

                    ui.add_space(15.0);
                    if ui.button("Конвектировать").clicked() {
                        self.is_convector = true;
                    }
                }
            } else {
                ui.label("Идет конвертация");
                egui::ScrollArea::vertical()
                    .max_height(120.0)
                    .show(ui, |ui| {
                        for (i, file) in self.path_files.iter().enumerate() {
                            let display_num = i + 1;
                            // УБРАЛИ ОТСЮДА self.name_music.push(...)

                            if self.is_name && i < self.name_music.len() {
                                ui.horizontal(|ui| {
                                    ui.label(format!("{display_num}. {file}"));
                                    ui.label(format!("Имя: {}", self.name_music[i]));
                                });
                            } else {
                                ui.label(format!("{display_num}. {file}"));
                            }
                        }
                    });
                
                // ИСПРАВЛЕННЫЙ ЦИКЛ КОНВЕРТАЦИИ
                while !self.path_files.is_empty() {
                    let original_path = self.path_files[0].as_str();
                    let path = Path::new(original_path);
                    
                    // Если имя не задано, берем оригинальное имя файла (без расширения)
                    let out_name = if self.name_music[0].is_empty() {
                        path.file_stem().unwrap_or_default().to_string_lossy().to_string()
                    } else {
                        self.name_music[0].clone()
                    };

                    let new_path = path.with_file_name(format!("{}.mp3", out_name));
                    let new_string = new_path.to_str().unwrap().to_string();

                    println!("Конвертирую: {} -> {}", self.path_files[0], new_string);

                    // ПРАВИЛЬНЫЙ ВЫЗОВ FFMPEG БЕЗ cmd /C
                    let output = Command::new("ffmpeg")
                        .args([
                            "-y",             // Перезаписывать файлы без вопросов
                            "-i", &self.path_files[0],
                            "-vn",            // Отключаем видеоряд
                            &new_string
                        ])
                        .output()
                        .expect("failed to execute process");
                    
                    if !output.status.success() {
                        println!("Ошибка ffmpeg: {}", String::from_utf8_lossy(&output.stderr));
                    }

                    // Удаляем обработанные элементы (всегда нулевой, так как мы в цикле while)
                    self.name_music.remove(0);
                    self.path_files.remove(0);
                }
                
                if self.path_files.is_empty() {
                    self.is_convector = false;
                }
            }
        });
    }
}