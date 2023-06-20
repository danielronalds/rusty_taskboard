//! This module contains the logic of the topbar
use egui::{containers::Frame, style::Margin, Color32, FontId, RichText, Rounding, Ui};

const TOPBAR_OUTER_MARGIN: f32 = 5.0;
const TOPBAR_OUTER_MARGIN_SIDE: f32 = 2.5;
const TOPBAR_INNER_MARGIN: f32 = 10.0;
const TOPBAR_ROUNDING: f32 = 5.0;
const TOPBAR_BORDER_WIDTH: f32 = 1.0;

/// The entry point for drawing the topbar
///
/// # Arguments
///
/// * `ui` - The UI to draw the topbar on
pub fn draw_topbar(ui: &mut Ui) {
    ui.horizontal(|ui| {
        draw_logo(ui);
        add_list(ui);
    });
}

/// This function draws the "logo"
///
/// # Arguments
///
/// * `ui` - The UI to draw the topbar on
fn draw_logo(ui: &mut Ui) {
    Frame::none()
        .fill(Color32::LIGHT_GRAY)
        .outer_margin(Margin::symmetric(
            TOPBAR_OUTER_MARGIN_SIDE,
            TOPBAR_OUTER_MARGIN,
        ))
        .rounding(Rounding::same(TOPBAR_ROUNDING))
        .show(ui, |ui| {
            Frame::none()
                .outer_margin(Margin::same(TOPBAR_BORDER_WIDTH))
                .inner_margin(Margin::same(TOPBAR_INNER_MARGIN))
                .rounding(Rounding::same(TOPBAR_ROUNDING))
                .fill(Color32::WHITE)
                .show(ui, |ui| {
                    ui.label(RichText::new("Rusty Taskboard").font(FontId::proportional(18.0)));
                });
        });
}

fn add_list(ui: &mut Ui) {
    Frame::none()
        .fill(Color32::LIGHT_GRAY)
        .outer_margin(Margin::symmetric(
            TOPBAR_OUTER_MARGIN_SIDE,
            TOPBAR_OUTER_MARGIN,
        ))
        .rounding(Rounding::same(TOPBAR_ROUNDING))
        .show(ui, |ui| {
            Frame::none()
                .outer_margin(Margin::same(TOPBAR_BORDER_WIDTH))
                .inner_margin(Margin::same(TOPBAR_INNER_MARGIN))
                .rounding(Rounding::same(TOPBAR_ROUNDING))
                .fill(Color32::WHITE)
                .show(ui, |ui| {
                    ui.label("Add List");
                    Frame::none()
                        .fill(Color32::LIGHT_GRAY)
                        .inner_margin(Margin::same(TOPBAR_BORDER_WIDTH))
                        .rounding(Rounding::same(TOPBAR_ROUNDING - 2.0))
                        .show(ui, |ui| ui.text_edit_singleline(&mut String::new()));
                });
        });
}
