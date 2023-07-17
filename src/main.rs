use egui_macroquad::*;
use macroquad::prelude::*;

#[derive(PartialEq, Eq)]
enum Panel {
    FourBand,
    FiveBand,
    SixBand,
    About,
}

#[derive(PartialEq, Debug)]
enum BandColor {
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
    Gray,
    White,
    Gold,
    Silver,
}

fn conf() -> Conf {
    let mut title = String::from("Resistor v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    Conf {
        window_title: title
        .to_owned(),
        fullscreen: false,
        sample_count: 16,
        window_width: 600,
        window_height: 400,
        window_resizable: false,
        ..Default::default()
    }
}

fn draw_resistor_body() {
    draw_circle(190.0, 88.0, 40.0, BEIGE);
    draw_circle(390.0, 88.0, 40.0, BEIGE);
    draw_rectangle(200.0, 53.0, 200.0, 69.0, BEIGE);
    draw_rectangle(70.0, 80.0, 80.0, 15.0, GRAY);
    draw_rectangle(430.0, 80.0, 80.0, 15.0, GRAY);
}

#[macroquad::main(conf)]
async fn main() {
    let mut panel: Panel = Panel::FiveBand;
    let mut first_band: BandColor = BandColor::Brown;
    let mut second_band: BandColor = BandColor::Black;
    let mut third_band: BandColor = BandColor::Black;
    let mut multiplier: BandColor = BandColor::Black;
    let mut tolerance: BandColor = BandColor::Brown;
    let mut temp_coefficient: BandColor = BandColor::Brown;
    
    let mut selected_first: String = "1".to_string();
    let mut selected_second: String = "0".to_string();
    let mut selected_third: String = "0".to_string();
    let mut selected_multiplier: i64 = 1;
    let mut string_multiplier: String = "Ω".to_string();
    let mut selected_tolerance: String = "±1% (F)".to_string();
    let mut selected_temp_coefficient: String = "100 ppm/ºC".to_string();
    let mut string_result: String = String::new();
    let mut multiplier_pos_x: f32;
    let mut tolerance_pos_x: f32;
    let mut int_result: f64 = 0.0;

    loop {
        clear_background(BLACK);
        
        egui_macroquad::ui(|ctx| {
            egui::CentralPanel::default().show(&ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut panel, Panel::FourBand, "4-Bands");
                    ui.selectable_value(&mut panel, Panel::FiveBand, "5-Bands");
                    ui.selectable_value(&mut panel, Panel::SixBand, "6-Bands");
                    ui.selectable_value(&mut panel, Panel::About, "About...");
                });
                
                if panel != Panel::About {
                    ui.add_space(120.0);
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("First ring");
                            egui::ComboBox::from_id_source("1").selected_text(format!("{:?}", first_band)).width(90.0).show_ui(ui, |ui| {
                                if ui.selectable_value(&mut first_band, BandColor::Brown, "Brown").clicked() {
                                    first_band = BandColor::Brown;
                                    selected_first = "1".to_string();
                                }
                                if ui.selectable_value(&mut first_band, BandColor::Red, "Red").clicked() {
                                    first_band = BandColor::Red;
                                    selected_first = "2".to_string();
                                }
                                if ui.selectable_value(&mut first_band, BandColor::Orange, "Orange").clicked() {
                                    first_band = BandColor::Orange;
                                    selected_first = "3".to_string();
                                }
                                if ui.selectable_value(&mut first_band, BandColor::Yellow, "Yellow").clicked() {
                                    first_band = BandColor::Yellow;
                                    selected_first = "4".to_string();
                                }
                                if ui.selectable_value(&mut first_band, BandColor::Green, "Green").clicked() {
                                    first_band = BandColor::Green;
                                    selected_first = "5".to_string();
                                }
                                if ui.selectable_value(&mut first_band, BandColor::Blue, "Blue").clicked() {
                                    first_band = BandColor::Blue;
                                    selected_first = "6".to_string();
                                }
                                if ui.selectable_value(&mut first_band, BandColor::Violet, "Violet").clicked() {
                                    first_band = BandColor::Violet;
                                    selected_first = "7".to_string();
                                }
                                if ui.selectable_value(&mut first_band, BandColor::Gray, "Gray").clicked() {
                                    first_band = BandColor::Gray;
                                    selected_first = "8".to_string();
                                }
                                if ui.selectable_value(&mut first_band, BandColor::White, "White").clicked() {
                                    first_band = BandColor::White;
                                    selected_first = "9".to_string();
                                }
                            });
                        });
                        ui.vertical(|ui| {
                            ui.label("Second ring");
                            egui::ComboBox::from_id_source("2").selected_text(format!("{:?}", second_band)).width(90.0).show_ui(ui, |ui| {
                                if ui.selectable_value(&mut second_band, BandColor::Black, "Black").clicked() {
                                    second_band = BandColor::Black;
                                    selected_second = "0".to_string();
                                }
                                if ui.selectable_value(&mut second_band, BandColor::Brown, "Brown").clicked() {
                                    second_band = BandColor::Brown;
                                    selected_second = "1".to_string();
                                }
                                if ui.selectable_value(&mut second_band, BandColor::Red, "Red").clicked() {
                                    second_band = BandColor::Red;
                                    selected_second = "2".to_string();
                                }
                                if ui.selectable_value(&mut second_band, BandColor::Orange, "Orange").clicked() {
                                    second_band = BandColor::Orange;
                                    selected_second = "3".to_string();
                                }
                                if ui.selectable_value(&mut second_band, BandColor::Yellow, "Yellow").clicked() {
                                    second_band = BandColor::Yellow;
                                    selected_second = "4".to_string();
                                }
                                if ui.selectable_value(&mut second_band, BandColor::Green, "Green").clicked() {
                                    second_band = BandColor::Green;
                                    selected_second = "5".to_string();
                                }
                                if ui.selectable_value(&mut second_band, BandColor::Blue, "Blue").clicked() {
                                    second_band = BandColor::Blue;
                                    selected_second = "6".to_string();
                                }
                                if ui.selectable_value(&mut second_band, BandColor::Violet, "Violet").clicked() {
                                    second_band = BandColor::Violet;
                                    selected_second = "7".to_string();
                                }
                                if ui.selectable_value(&mut second_band, BandColor::Gray, "Gray").clicked() {
                                    second_band = BandColor::Gray;
                                    selected_second = "8".to_string();
                                }
                                if ui.selectable_value(&mut second_band, BandColor::White, "White").clicked() {
                                    second_band = BandColor::White;
                                    selected_second = "9".to_string();
                                }
                            });
                        });
                        if panel == Panel::FiveBand || panel == Panel::SixBand {
                            ui.vertical(|ui| {
                                ui.label("Third ring");
                                egui::ComboBox::from_id_source("3").selected_text(format!("{:?}", third_band)).width(90.0).show_ui(ui, |ui| {
                                    if ui.selectable_value(&mut third_band, BandColor::Black, "Black").clicked() {
                                        third_band = BandColor::Black;
                                        selected_third = "0".to_string();
                                    }
                                    if ui.selectable_value(&mut third_band, BandColor::Brown, "Brown").clicked() {
                                        third_band = BandColor::Brown;
                                        selected_third = "1".to_string();
                                    }
                                    if ui.selectable_value(&mut third_band, BandColor::Red, "Red").clicked() {
                                        third_band = BandColor::Red;
                                        selected_third = "2".to_string();
                                    }
                                    if ui.selectable_value(&mut third_band, BandColor::Orange, "Orange").clicked() {
                                        third_band = BandColor::Orange;
                                        selected_third = "3".to_string();
                                    }
                                    if ui.selectable_value(&mut third_band, BandColor::Yellow, "Yellow").clicked() {
                                        third_band = BandColor::Yellow;
                                        selected_third = "4".to_string();
                                    }
                                    if ui.selectable_value(&mut third_band, BandColor::Green, "Green").clicked() {
                                        third_band = BandColor::Green;
                                        selected_third = "5".to_string();
                                    }
                                    if ui.selectable_value(&mut third_band, BandColor::Blue, "Blue").clicked() {
                                        third_band = BandColor::Blue;
                                        selected_third = "6".to_string();
                                    }
                                    if ui.selectable_value(&mut third_band, BandColor::Violet, "Violet").clicked() {
                                        third_band = BandColor::Violet;
                                        selected_third = "7".to_string();
                                    }
                                    if ui.selectable_value(&mut third_band, BandColor::Gray, "Gray").clicked() {
                                        third_band = BandColor::Gray;
                                        selected_third = "8".to_string();
                                    }
                                    if ui.selectable_value(&mut third_band, BandColor::White, "White").clicked() {
                                        third_band = BandColor::White;
                                        selected_third = "9".to_string();
                                    }
                                });
                            });
                        }
                        ui.vertical(|ui| {
                            ui.label("Multiplier ring");
                            egui::ComboBox::from_id_source("4").selected_text(format!("{:?}", multiplier)).width(90.0).show_ui(ui, |ui| {
                                if ui.selectable_value(&mut multiplier, BandColor::Black, "Black").clicked() {
                                    multiplier = BandColor::Black;
                                    selected_multiplier = 1;
                                }
                                if ui.selectable_value(&mut multiplier, BandColor::Brown, "Brown").clicked() {
                                    multiplier = BandColor::Brown;
                                    selected_multiplier = 10;
                                }
                                if ui.selectable_value(&mut multiplier, BandColor::Red, "Red").clicked() {
                                    multiplier = BandColor::Red;
                                    selected_multiplier = 100;
                                }
                                if ui.selectable_value(&mut multiplier, BandColor::Orange, "Orange").clicked() {
                                    multiplier = BandColor::Orange;
                                    selected_multiplier = 1000;
                                }
                                if ui.selectable_value(&mut multiplier, BandColor::Yellow, "Yellow").clicked() {
                                    multiplier = BandColor::Yellow;
                                    selected_multiplier = 10000;
                                }
                                if ui.selectable_value(&mut multiplier, BandColor::Green, "Green").clicked() {
                                    multiplier = BandColor::Green;
                                    selected_multiplier = 100000;
                                }
                                if ui.selectable_value(&mut multiplier, BandColor::Blue, "Blue").clicked() {
                                    multiplier = BandColor::Blue;
                                    selected_multiplier = 1000000;
                                }
                                if ui.selectable_value(&mut multiplier, BandColor::Violet, "Violet").clicked() {
                                    multiplier = BandColor::Violet;
                                    selected_multiplier = 10000000;
                                }
                                if ui.selectable_value(&mut multiplier, BandColor::Gray, "Gray").clicked() {
                                    multiplier = BandColor::Gray;
                                    selected_multiplier = 100000000;
                                }
                                if ui.selectable_value(&mut multiplier, BandColor::White, "White").clicked() {
                                    multiplier = BandColor::White;
                                    selected_multiplier = 1000000000;
                                }
                            });
                        });
                        ui.vertical(|ui| {
                            ui.label("Tolerance ring");
                            egui::ComboBox::from_id_source("5").selected_text(format!("{:?}", tolerance)).width(90.0).show_ui(ui, |ui| {
                                if ui.selectable_value(&mut tolerance, BandColor::Brown, "Brown").clicked() {
                                    tolerance = BandColor::Brown;
                                    selected_tolerance = "±1% (F)".to_string();
                                }
                                if ui.selectable_value(&mut tolerance, BandColor::Red, "Red").clicked() {
                                    tolerance = BandColor::Red;
                                    selected_tolerance = "±2% (G)".to_string();
                                }
                                if ui.selectable_value(&mut tolerance, BandColor::Orange, "Orange").clicked() {
                                    tolerance = BandColor::Orange;
                                    selected_tolerance = "±0.05% (W)".to_string();
                                }
                                if ui.selectable_value(&mut tolerance, BandColor::Yellow, "Yellow").clicked() {
                                    tolerance = BandColor::Yellow;
                                    selected_tolerance = "±0.02% (P)".to_string();
                                }
                                if ui.selectable_value(&mut tolerance, BandColor::Green, "Green").clicked() {
                                    tolerance = BandColor::Green;
                                    selected_tolerance = "±0.5% (D)".to_string();
                                }
                                if ui.selectable_value(&mut tolerance, BandColor::Blue, "Blue").clicked() {
                                    tolerance = BandColor::Blue;
                                    selected_tolerance = "±0.25% (C)".to_string();
                                }
                                if ui.selectable_value(&mut tolerance, BandColor::Violet, "Violet").clicked() {
                                    tolerance = BandColor::Violet;
                                    selected_tolerance = "±0.1% (B)".to_string();
                                }
                                if ui.selectable_value(&mut tolerance, BandColor::Gray, "Gray").clicked() {
                                    tolerance = BandColor::Gray;
                                    selected_tolerance = "±0.01% (L)".to_string();
                                }
                                if ui.selectable_value(&mut tolerance, BandColor::Gold, "Gold").clicked() {
                                    tolerance = BandColor::Gold;
                                    selected_tolerance = "±5% (J)".to_string();
                                }
                                if ui.selectable_value(&mut tolerance, BandColor::Silver, "Silver").clicked() {
                                    tolerance = BandColor::Silver;
                                    selected_tolerance = "±10% (K)".to_string();
                                }
                            });
                        });
                        if panel == Panel::SixBand {
                            ui.vertical(|ui| {
                                ui.label("Temp. coeff.");
                                egui::ComboBox::from_id_source("6").selected_text(format!("{:?}", temp_coefficient)).width(90.0).show_ui(ui, |ui| {
                                    if ui.selectable_value(&mut temp_coefficient, BandColor::Brown, "Brown").clicked() {
                                        temp_coefficient = BandColor::Brown;
                                        selected_temp_coefficient = "100 ppm/ºC".to_string();
                                    }
                                    if ui.selectable_value(&mut temp_coefficient, BandColor::Red, "Red").clicked() {
                                        temp_coefficient = BandColor::Red;
                                        selected_temp_coefficient = "50 ppm/ºC".to_string();
                                    }
                                    if ui.selectable_value(&mut temp_coefficient, BandColor::Orange, "Orange").clicked() {
                                        temp_coefficient = BandColor::Orange;
                                        selected_temp_coefficient = "15 ppm/ºC".to_string();
                                    }
                                    if ui.selectable_value(&mut temp_coefficient, BandColor::Yellow, "Yellow").clicked() {
                                        temp_coefficient = BandColor::Yellow;
                                        selected_temp_coefficient = "25 ppm/ºC".to_string();
                                    }
                                    if ui.selectable_value(&mut temp_coefficient, BandColor::Blue, "Blue").clicked() {
                                        temp_coefficient = BandColor::Blue;
                                        selected_temp_coefficient = "10 ppm/ºC".to_string();
                                    }
                                    if ui.selectable_value(&mut temp_coefficient, BandColor::Violet, "Violet").clicked() {
                                        temp_coefficient = BandColor::Violet;
                                        selected_temp_coefficient = "5 ppm/ºC".to_string();
                                    }
                                    if ui.selectable_value(&mut temp_coefficient, BandColor::White, "White").clicked() {
                                        temp_coefficient = BandColor::White;
                                        selected_temp_coefficient = "1 ppm/ºC".to_string();
                                    }
                                });
                            });
                        }
                    });
                    ui.separator();

                    match panel {
                        Panel::FourBand => {
                            string_result.push_str(&selected_first);
                            string_result.push_str(&selected_second);
                        },
                        Panel::FiveBand => {
                            string_result.push_str(&selected_first);
                            string_result.push_str(&selected_second);
                            string_result.push_str(&selected_third);
                        },
                        Panel::SixBand => {
                            string_result.push_str(&selected_first);
                            string_result.push_str(&selected_second);
                            string_result.push_str(&selected_third);
                        },
                        Panel::About => {},
                    }

                    int_result = string_result.parse::<f64>().unwrap() * selected_multiplier as f64;

                    if int_result >= 100000000.0 {
                        int_result = int_result / 1000000000.0;
                        string_multiplier = "GΩ".to_string();
                    } else if int_result >= 10000000.0 {
                        int_result = int_result / 1000000.0;
                        string_multiplier = "MΩ".to_string();
                    } else if int_result >= 1000000.0 {
                        int_result = int_result / 1000000.0;
                        string_multiplier = "MΩ".to_string();
                    } else if int_result >= 100000.0 {
                        int_result = int_result / 1000.0;
                        string_multiplier = "kΩ".to_string();
                    } else if int_result >= 10000.0 {
                        int_result = int_result / 1000.0;
                        string_multiplier = "kΩ".to_string();
                    } else if int_result >= 1000.0 {
                        int_result = int_result / 1000.0;
                        string_multiplier = "kΩ".to_string();
                    } else if int_result >= 100.0 {
                        string_multiplier = "Ω".to_string();
                    }

                
                    ui.centered_and_justified(|ui| {
                        if panel == Panel::SixBand {
                            ui.label(egui::RichText::new(format!("{} {} {} {}", int_result.to_string(), string_multiplier, selected_tolerance, selected_temp_coefficient)).font(egui::FontId::proportional(40.0)).color(egui::Color32::WHITE));
                        } else {
                            ui.label(egui::RichText::new(format!("{} {} {}", int_result.to_string(), string_multiplier, selected_tolerance)).font(egui::FontId::proportional(40.0)).color(egui::Color32::WHITE));
                        }
                    });

                    ui.add_space(-25.0);
                    ui.separator();
                    ui.hyperlink("https://en.wikipedia.org/wiki/Resistor#Resistor_marking");

                    string_result.clear();
                } else {
                    ui.add_space(20.0);
                    ui.centered_and_justified(|ui| {
                        ui.label(egui::RichText::new("by   -=De/\\/=-   2023y.").font(egui::FontId::proportional(20.0)).color(egui::Color32::GREEN));
                    });
                }
            });
        });

        egui_macroquad::draw();

        if panel != Panel::About {
            draw_resistor_body();
            
            match selected_first.as_str() {
                "1" => {draw_rectangle(187.0, 48.0, 6.0, 80.0, DARKBROWN)}
                "2" => {draw_rectangle(187.0, 48.0, 6.0, 80.0, RED)}
                "3" => {draw_rectangle(187.0, 48.0, 6.0, 80.0, ORANGE)}
                "4" => {draw_rectangle(187.0, 48.0, 6.0, 80.0, YELLOW)}
                "5" => {draw_rectangle(187.0, 48.0, 6.0, 80.0, GREEN)}
                "6" => {draw_rectangle(187.0, 48.0, 6.0, 80.0, BLUE)}
                "7" => {draw_rectangle(187.0, 48.0, 6.0, 80.0, VIOLET)}
                "8" => {draw_rectangle(187.0, 48.0, 6.0, 80.0, GRAY)}
                _ => {draw_rectangle(187.0, 48.0, 6.0, 80.0, WHITE)}
            }

            match selected_second.as_str() {
                "0" => {draw_rectangle(220.0, 53.0, 6.0, 69.0, BLACK)}
                "1" => {draw_rectangle(220.0, 53.0, 6.0, 69.0, DARKBROWN)}
                "2" => {draw_rectangle(220.0, 53.0, 6.0, 69.0, RED)}
                "3" => {draw_rectangle(220.0, 53.0, 6.0, 69.0, ORANGE)}
                "4" => {draw_rectangle(220.0, 53.0, 6.0, 69.0, YELLOW)}
                "5" => {draw_rectangle(220.0, 53.0, 6.0, 69.0, GREEN)}
                "6" => {draw_rectangle(220.0, 53.0, 6.0, 69.0, BLUE)}
                "7" => {draw_rectangle(220.0, 53.0, 6.0, 69.0, VIOLET)}
                "8" => {draw_rectangle(220.0, 53.0, 6.0, 69.0, GRAY)}
                _ => {draw_rectangle(220.0, 53.0, 6.0, 69.0, WHITE)}
            }

            if panel == Panel::FiveBand || panel == Panel::SixBand {
                match selected_third.as_str() {
                    "0" => {draw_rectangle(250.0, 53.0, 6.0, 69.0, BLACK)}
                    "1" => {draw_rectangle(250.0, 53.0, 6.0, 69.0, DARKBROWN)}
                    "2" => {draw_rectangle(250.0, 53.0, 6.0, 69.0, RED)}
                    "3" => {draw_rectangle(250.0, 53.0, 6.0, 69.0, ORANGE)}
                    "4" => {draw_rectangle(250.0, 53.0, 6.0, 69.0, YELLOW)}
                    "5" => {draw_rectangle(250.0, 53.0, 6.0, 69.0, GREEN)}
                    "6" => {draw_rectangle(250.0, 53.0, 6.0, 69.0, BLUE)}
                    "7" => {draw_rectangle(250.0, 53.0, 6.0, 69.0, VIOLET)}
                    "8" => {draw_rectangle(250.0, 53.0, 6.0, 69.0, GRAY)}
                    "9" => {draw_rectangle(250.0, 53.0, 6.0, 69.0, WHITE)}
                    _ => {}
                }
                multiplier_pos_x = 280.0;
                tolerance_pos_x = 310.0;
            } else {
                multiplier_pos_x = 250.0;
                tolerance_pos_x = 280.0;
            }

            match multiplier {
                BandColor::Black => {draw_rectangle(multiplier_pos_x, 53.0, 6.0, 69.0, BLACK)},
                BandColor::Brown => {draw_rectangle(multiplier_pos_x, 53.0, 6.0, 69.0, DARKBROWN)}
                BandColor::Red => {draw_rectangle(multiplier_pos_x, 53.0, 6.0, 69.0, RED)}
                BandColor::Orange => {draw_rectangle(multiplier_pos_x, 53.0, 6.0, 69.0, ORANGE)}
                BandColor::Yellow => {draw_rectangle(multiplier_pos_x, 53.0, 6.0, 69.0, YELLOW)}
                BandColor::Green => {draw_rectangle(multiplier_pos_x, 53.0, 6.0, 69.0, GREEN)}
                BandColor::Blue => {draw_rectangle(multiplier_pos_x, 53.0, 6.0, 69.0, BLUE)}
                BandColor::Violet => {draw_rectangle(multiplier_pos_x, 53.0, 6.0, 69.0, VIOLET)}
                BandColor::Gray => {draw_rectangle(multiplier_pos_x, 53.0, 6.0, 69.0, GRAY)}
                BandColor::White => {draw_rectangle(multiplier_pos_x, 53.0, 6.0, 69.0, WHITE)}
                BandColor::Gold => {draw_rectangle(multiplier_pos_x, 53.0, 6.0, 69.0, GOLD)}
                BandColor::Silver => {draw_rectangle(multiplier_pos_x, 53.0, 6.0, 69.0, LIGHTGRAY)}
            }

            match tolerance {
                BandColor::Black => {draw_rectangle(tolerance_pos_x, 53.0, 6.0, 69.0, BLACK)},
                BandColor::Brown => {draw_rectangle(tolerance_pos_x, 53.0, 6.0, 69.0, DARKBROWN)}
                BandColor::Red => {draw_rectangle(tolerance_pos_x, 53.0, 6.0, 69.0, RED)}
                BandColor::Orange => {draw_rectangle(tolerance_pos_x, 53.0, 6.0, 69.0, ORANGE)}
                BandColor::Yellow => {draw_rectangle(tolerance_pos_x, 53.0, 6.0, 69.0, YELLOW)}
                BandColor::Green => {draw_rectangle(tolerance_pos_x, 53.0, 6.0, 69.0, GREEN)}
                BandColor::Blue => {draw_rectangle(tolerance_pos_x, 53.0, 6.0, 69.0, BLUE)}
                BandColor::Violet => {draw_rectangle(tolerance_pos_x, 53.0, 6.0, 69.0, VIOLET)}
                BandColor::Gray => {draw_rectangle(tolerance_pos_x, 53.0, 6.0, 69.0, GRAY)}
                BandColor::White => {draw_rectangle(tolerance_pos_x, 53.0, 6.0, 69.0, WHITE)}
                BandColor::Gold => {draw_rectangle(tolerance_pos_x, 53.0, 6.0, 69.0, GOLD)}
                BandColor::Silver => {draw_rectangle(tolerance_pos_x, 53.0, 6.0, 69.0, LIGHTGRAY)}
            }

            if panel == Panel::SixBand {
                match temp_coefficient {
                    BandColor::Black => {draw_rectangle(387.0, 48.0, 6.0, 80.0, BLACK)},
                    BandColor::Brown => {draw_rectangle(387.0, 48.0, 6.0, 80.0, DARKBROWN)}
                    BandColor::Red => {draw_rectangle(387.0, 48.0, 6.0, 80.0, RED)}
                    BandColor::Orange => {draw_rectangle(387.0, 48.0, 6.0, 80.0, ORANGE)}
                    BandColor::Yellow => {draw_rectangle(387.0, 48.0, 6.0, 80.0, YELLOW)}
                    BandColor::Green => {draw_rectangle(387.0, 48.0, 6.0, 80.0, GREEN)}
                    BandColor::Blue => {draw_rectangle(387.0, 48.0, 6.0, 80.0, BLUE)}
                    BandColor::Violet => {draw_rectangle(387.0, 48.0, 6.0, 80.0, VIOLET)}
                    BandColor::Gray => {draw_rectangle(387.0, 48.0, 6.0, 80.0, GRAY)}
                    BandColor::White => {draw_rectangle(387.0, 48.0, 6.0, 80.0, WHITE)}
                    BandColor::Gold => {draw_rectangle(387.0, 48.0, 6.0, 80.0, GOLD)}
                    BandColor::Silver => {draw_rectangle(387.0, 48.0, 6.0, 80.0, LIGHTGRAY)}
                }
            }
        }

        next_frame().await
    }
}