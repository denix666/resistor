#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui::{Color32, CornerRadius, Pos2, Rect, Vec2};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// BandColor
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
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

impl BandColor {
    fn digit(self) -> u8 {
        match self {
            Self::Black => 0,
            Self::Brown => 1,
            Self::Red => 2,
            Self::Orange => 3,
            Self::Yellow => 4,
            Self::Green => 5,
            Self::Blue => 6,
            Self::Violet => 7,
            Self::Gray => 8,
            Self::White => 9,
            Self::Gold | Self::Silver => 0,
        }
    }

    fn multiplier(self) -> f64 {
        match self {
            Self::Black => 1.0,
            Self::Brown => 10.0,
            Self::Red => 100.0,
            Self::Orange => 1_000.0,
            Self::Yellow => 10_000.0,
            Self::Green => 100_000.0,
            Self::Blue => 1_000_000.0,
            Self::Violet => 10_000_000.0,
            Self::Gray => 100_000_000.0,
            Self::White => 1_000_000_000.0,
            Self::Gold => 0.1,
            Self::Silver => 0.01,
        }
    }

    fn tolerance(self) -> &'static str {
        match self {
            Self::Brown => "±1% (F)",
            Self::Red => "±2% (G)",
            Self::Orange => "±0.05% (W)",
            Self::Yellow => "±0.02% (P)",
            Self::Green => "±0.5% (D)",
            Self::Blue => "±0.25% (C)",
            Self::Violet => "±0.1% (B)",
            Self::Gray => "±0.01% (L)",
            Self::Gold => "±5% (J)",
            Self::Silver => "±10% (K)",
            _ => "",
        }
    }

    fn temp_coefficient(self) -> &'static str {
        match self {
            Self::Brown => "100 ppm/ºC",
            Self::Red => "50 ppm/ºC",
            Self::Orange => "15 ppm/ºC",
            Self::Yellow => "25 ppm/ºC",
            Self::Blue => "10 ppm/ºC",
            Self::Violet => "5 ppm/ºC",
            Self::White => "1 ppm/ºC",
            _ => "",
        }
    }

    fn color32(self) -> Color32 {
        match self {
            Self::Black => Color32::from_rgb(10, 10, 10),
            Self::Brown => Color32::from_rgb(139, 69, 19),
            Self::Red => Color32::from_rgb(220, 20, 20),
            Self::Orange => Color32::from_rgb(255, 140, 0),
            Self::Yellow => Color32::from_rgb(255, 255, 0),
            Self::Green => Color32::from_rgb(0, 160, 0),
            Self::Blue => Color32::from_rgb(0, 0, 220),
            Self::Violet => Color32::from_rgb(148, 0, 211),
            Self::Gray => Color32::from_rgb(128, 128, 128),
            Self::White => Color32::from_rgb(255, 255, 255),
            Self::Gold => Color32::from_rgb(218, 165, 32),
            Self::Silver => Color32::from_rgb(192, 192, 192),
        }
    }

    const FIRST_DIGIT: &[Self] = &[
        Self::Brown,
        Self::Red,
        Self::Orange,
        Self::Yellow,
        Self::Green,
        Self::Blue,
        Self::Violet,
        Self::Gray,
        Self::White,
    ];

    const DIGIT: &[Self] = &[
        Self::Black,
        Self::Brown,
        Self::Red,
        Self::Orange,
        Self::Yellow,
        Self::Green,
        Self::Blue,
        Self::Violet,
        Self::Gray,
        Self::White,
    ];

    const MULTIPLIER: &[Self] = &[
        Self::Black,
        Self::Brown,
        Self::Red,
        Self::Orange,
        Self::Yellow,
        Self::Green,
        Self::Blue,
        Self::Violet,
        Self::Gray,
        Self::White,
        Self::Gold,
        Self::Silver,
    ];

    const TOLERANCE: &[Self] = &[
        Self::Brown,
        Self::Red,
        Self::Orange,
        Self::Yellow,
        Self::Green,
        Self::Blue,
        Self::Violet,
        Self::Gray,
        Self::Gold,
        Self::Silver,
    ];

    const TEMP_COEFFICIENT: &[Self] = &[
        Self::Brown,
        Self::Red,
        Self::Orange,
        Self::Yellow,
        Self::Blue,
        Self::Violet,
        Self::White,
    ];
}

impl std::fmt::Display for BandColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

// ---------------------------------------------------------------------------
// Panel
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
enum Panel {
    FourBand,
    FiveBand,
    SixBand,
    About,
}

// ---------------------------------------------------------------------------
// Config (persisted to JSON)
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(default)]
    window_pos: Option<[f32; 2]>,
    #[serde(default = "default_panel")]
    panel: Panel,
    #[serde(default = "default_first")]
    first_band: BandColor,
    #[serde(default = "default_zero")]
    second_band: BandColor,
    #[serde(default = "default_zero")]
    third_band: BandColor,
    #[serde(default = "default_zero")]
    multiplier: BandColor,
    #[serde(default = "default_tolerance")]
    tolerance: BandColor,
    #[serde(default = "default_tolerance")]
    temp_coefficient: BandColor,
}

fn default_panel() -> Panel {
    Panel::FiveBand
}
fn default_first() -> BandColor {
    BandColor::Brown
}
fn default_zero() -> BandColor {
    BandColor::Black
}
fn default_tolerance() -> BandColor {
    BandColor::Brown
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_pos: None,
            panel: default_panel(),
            first_band: default_first(),
            second_band: default_zero(),
            third_band: default_zero(),
            multiplier: default_zero(),
            tolerance: default_tolerance(),
            temp_coefficient: default_tolerance(),
        }
    }
}

impl Config {
    fn path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("resistor")
            .join("config.json")
    }

    fn load() -> Self {
        std::fs::read_to_string(Self::path())
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    fn save(&self) {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(path, json);
        }
    }
}

// ---------------------------------------------------------------------------
// App
// ---------------------------------------------------------------------------

struct ResistorApp {
    panel: Panel,
    first_band: BandColor,
    second_band: BandColor,
    third_band: BandColor,
    multiplier: BandColor,
    tolerance: BandColor,
    temp_coefficient: BandColor,
    window_pos: Option<[f32; 2]>,
}

impl ResistorApp {
    fn from_config(cfg: &Config) -> Self {
        Self {
            panel: cfg.panel,
            first_band: cfg.first_band,
            second_band: cfg.second_band,
            third_band: cfg.third_band,
            multiplier: cfg.multiplier,
            tolerance: cfg.tolerance,
            temp_coefficient: cfg.temp_coefficient,
            window_pos: cfg.window_pos,
        }
    }

    fn save_config(&self) {
        Config {
            window_pos: self.window_pos,
            panel: self.panel,
            first_band: self.first_band,
            second_band: self.second_band,
            third_band: self.third_band,
            multiplier: self.multiplier,
            tolerance: self.tolerance,
            temp_coefficient: self.temp_coefficient,
        }
        .save();
    }

    fn resistance(&self) -> f64 {
        let base = match self.panel {
            Panel::FourBand => {
                self.first_band.digit() as f64 * 10.0 + self.second_band.digit() as f64
            }
            Panel::FiveBand | Panel::SixBand => {
                self.first_band.digit() as f64 * 100.0
                    + self.second_band.digit() as f64 * 10.0
                    + self.third_band.digit() as f64
            }
            Panel::About => 0.0,
        };
        base * self.multiplier.multiplier()
    }

    fn format_resistance(&self) -> String {
        let ohms = self.resistance();
        let (val, unit) = if ohms >= 1_000_000_000.0 {
            (ohms / 1_000_000_000.0, "GΩ")
        } else if ohms >= 1_000_000.0 {
            (ohms / 1_000_000.0, "MΩ")
        } else if ohms >= 1_000.0 {
            (ohms / 1_000.0, "kΩ")
        } else {
            (ohms, "Ω")
        };

        let formatted = if val == val.floor() {
            format!("{}", val as i64)
        } else {
            format!("{val}")
        };

        let tol = self.tolerance.tolerance();
        if self.panel == Panel::SixBand {
            format!(
                "{formatted} {unit}  {tol}  {}",
                self.temp_coefficient.temp_coefficient()
            )
        } else {
            format!("{formatted} {unit}  {tol}")
        }
    }

    fn band_colors(&self) -> Vec<Color32> {
        match self.panel {
            Panel::FourBand => vec![
                self.first_band.color32(),
                self.second_band.color32(),
                self.multiplier.color32(),
                self.tolerance.color32(),
            ],
            Panel::FiveBand => vec![
                self.first_band.color32(),
                self.second_band.color32(),
                self.third_band.color32(),
                self.multiplier.color32(),
                self.tolerance.color32(),
            ],
            Panel::SixBand => vec![
                self.first_band.color32(),
                self.second_band.color32(),
                self.third_band.color32(),
                self.multiplier.color32(),
                self.tolerance.color32(),
                self.temp_coefficient.color32(),
            ],
            Panel::About => vec![],
        }
    }
}

// ---------------------------------------------------------------------------
// Drawing helpers
// ---------------------------------------------------------------------------

fn draw_resistor(ui: &mut egui::Ui, app: &ResistorApp) {
    let width = ui.available_width();
    let height = 90.0;
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::hover());
    let painter = ui.painter_at(rect);

    let cx = rect.center().x;
    let cy = rect.center().y;
    let body_w = 230.0;
    let body_h = 50.0;
    let bulge_r = 30.0;
    let lead_w = 80.0;
    let lead_h = 8.0;
    let lead_color = Color32::from_rgb(160, 160, 160);
    let body_color = Color32::from_rgb(200, 170, 130);

    let left_cap = cx - body_w / 2.0;
    let right_cap = cx + body_w / 2.0;

    // Leads
    painter.rect_filled(
        Rect::from_min_size(
            Pos2::new(left_cap - lead_w, cy - lead_h / 2.0),
            Vec2::new(lead_w, lead_h),
        ),
        CornerRadius::ZERO,
        lead_color,
    );
    painter.rect_filled(
        Rect::from_min_size(
            Pos2::new(right_cap, cy - lead_h / 2.0),
            Vec2::new(lead_w, lead_h),
        ),
        CornerRadius::ZERO,
        lead_color,
    );

    // End-cap bulges
    painter.circle_filled(Pos2::new(left_cap, cy), bulge_r, body_color);
    painter.circle_filled(Pos2::new(right_cap, cy), bulge_r, body_color);

    // Main body
    painter.rect_filled(
        Rect::from_center_size(Pos2::new(cx, cy), Vec2::new(body_w, body_h)),
        CornerRadius::ZERO,
        body_color,
    );

    // Color bands
    let bands = app.band_colors();
    if bands.is_empty() {
        return;
    }
    let n = bands.len() as f32;
    let band_h = bulge_r * 2.0;

    // First band on the left bulge, last band (6-band mode) on the right bulge,
    // remaining bands evenly spaced on the body
    let first_x = left_cap;
    let last_on_bulge = app.panel == Panel::SixBand;
    let last_x = right_cap;

    let inner_bands = if last_on_bulge { &bands[1..n as usize - 1] } else { &bands[1..] };
    let inner_n = inner_bands.len() as f32;
    let zone = body_w * 0.70;
    let spacing = zone / (inner_n + 1.0);
    let start_x = cx - zone / 2.0;

    // First band (on left bulge)
    painter.rect_filled(
        Rect::from_center_size(Pos2::new(first_x, cy), Vec2::new(7.0, band_h)),
        CornerRadius::ZERO,
        bands[0],
    );

    // Inner bands (on body)
    for (i, color) in inner_bands.iter().enumerate() {
        let x = start_x + spacing * (i as f32 + 1.0);
        painter.rect_filled(
            Rect::from_center_size(Pos2::new(x, cy), Vec2::new(7.0, body_h)),
            CornerRadius::ZERO,
            *color,
        );
    }

    // Last band on right bulge (6-band only)
    if last_on_bulge {
        painter.rect_filled(
            Rect::from_center_size(Pos2::new(last_x, cy), Vec2::new(7.0, band_h)),
            CornerRadius::ZERO,
            *bands.last().unwrap(),
        );
    }
}

fn band_combo(
    ui: &mut egui::Ui,
    id: &str,
    label: &str,
    selected: &mut BandColor,
    options: &[BandColor],
) {
    ui.vertical(|ui| {
        ui.label(label);
        egui::ComboBox::from_id_salt(id)
            .selected_text(format!("{selected}"))
            .width(90.0)
            .show_ui(ui, |ui| {
                for &color in options {
                    ui.selectable_value(selected, color, format!("{color}"));
                }
            });
    });
}

// ---------------------------------------------------------------------------
// eframe::App
// ---------------------------------------------------------------------------

impl eframe::App for ResistorApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if let Some(outer) = ui.ctx().input(|i| i.viewport().outer_rect) {
            self.window_pos = Some([outer.min.x, outer.min.y]);
        }

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.panel, Panel::FourBand, "4-Bands");
            ui.selectable_value(&mut self.panel, Panel::FiveBand, "5-Bands");
            ui.selectable_value(&mut self.panel, Panel::SixBand, "6-Bands");
            ui.selectable_value(&mut self.panel, Panel::About, "About");
        });

        if self.panel == Panel::About {
            ui.add_space(40.0);
            ui.centered_and_justified(|ui| {
                ui.label(
                    egui::RichText::new("by   -=De/\\/=-   2023y.")
                        .size(20.0)
                        .color(Color32::from_rgb(0, 200, 0)),
                );
            });
            return;
        }

        ui.add_space(4.0);
        draw_resistor(ui, self);
        ui.add_space(4.0);
        ui.separator();

        ui.horizontal(|ui| {
            band_combo(
                ui,
                "first",
                "1st band",
                &mut self.first_band,
                BandColor::FIRST_DIGIT,
            );
            band_combo(
                ui,
                "second",
                "2nd band",
                &mut self.second_band,
                BandColor::DIGIT,
            );
            if self.panel == Panel::FiveBand || self.panel == Panel::SixBand {
                band_combo(
                    ui,
                    "third",
                    "3rd band",
                    &mut self.third_band,
                    BandColor::DIGIT,
                );
            }
            band_combo(
                ui,
                "multiplier",
                "Multiplier",
                &mut self.multiplier,
                BandColor::MULTIPLIER,
            );
            band_combo(
                ui,
                "tolerance",
                "Tolerance",
                &mut self.tolerance,
                BandColor::TOLERANCE,
            );
            if self.panel == Panel::SixBand {
                band_combo(
                    ui,
                    "temp",
                    "Temp. coeff.",
                    &mut self.temp_coefficient,
                    BandColor::TEMP_COEFFICIENT,
                );
            }
        });

        ui.separator();

        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.label(
                egui::RichText::new(self.format_resistance())
                    .size(34.0)
                    .color(Color32::WHITE),
            );
            ui.add_space(10.0);
        });

        ui.separator();
        ui.hyperlink("https://en.wikipedia.org/wiki/Resistor#Resistor_marking");
    }

    fn on_exit(&mut self) {
        self.save_config();
    }
}

impl Drop for ResistorApp {
    fn drop(&mut self) {
        self.save_config();
    }
}

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------

fn main() -> eframe::Result<()> {
    let config = Config::load();

    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([600.0, 400.0])
        .with_resizable(false)
        .with_title(format!("Resistor v{}", env!("CARGO_PKG_VERSION")));

    if let Some([x, y]) = config.window_pos {
        viewport = viewport.with_position([x, y]);
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Resistor",
        options,
        Box::new(|_cc| Ok(Box::new(ResistorApp::from_config(&config)))),
    )
}
