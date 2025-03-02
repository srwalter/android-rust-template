use eframe::{
    egui::{self, Button, Label, Margin, RichText, ScrollArea},
    CreationContext,
};

use crate::PlatformContext;

mod controller;
use controller::{Controller, OutgoingMessage};

pub struct App {
    _platform_ctx: PlatformContext,
    content_margin: Margin,
    ctrl: Controller,
    msg: Vec<String>,
}

impl App {
    pub const NAME: &'static str = "Rust App";

    pub fn new(ctx: &CreationContext, platform_ctx: PlatformContext) -> Self {
        log::info!("detected OS: {:?}", ctx.egui_ctx.os());
        log::info!("screen PPI: {}", ctx.egui_ctx.pixels_per_point());

        let content_margin = Margin::default();
        let ctrl = Controller::new();

        Self {
            _platform_ctx: platform_ctx,
            content_margin,
            ctrl,
            msg: vec![],
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Some(msg) = self.ctrl.try_recv() {
            match msg {
                OutgoingMessage::Toast(msg) => self.msg.push(msg),
            }
        }

        // Scale up everything by 50%
        if let Some(ppp) = frame.info().native_pixels_per_point {
            ctx.set_pixels_per_point(ppp * 1.5);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Account for mobile display margins
            ui.add_space(self.content_margin.top);

            ui.add(
                Label::new(
                    RichText::new(format!("{}", "Hello world"))
                        .size(60.0)
                        .color(ui.visuals().strong_text_color()),
                ),
            );

            if ui.add(Button::new("Poke")).clicked() {
                self.ctrl.poke();
            }

            ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                for m in &self.msg {
                    ui.add(
                        Label::new(
                            RichText::new(m)
                                .size(30.0)
                                .color(ui.visuals().strong_text_color()),
                        ),
                    );
                }
            });
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
    }
}
