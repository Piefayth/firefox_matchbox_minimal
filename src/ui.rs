use crate::common::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_system_set(SystemSet::on_update(GameState::Login).with_system(login_ui))
            .add_system_set(SystemSet::on_update(GameState::Lobby).with_system(network_debug_ui));
    }
}

fn login_ui(
    mut egui_context: ResMut<EguiContext>,
    mut login_state: ResMut<LoginState>,
    mut game_state: ResMut<State<GameState>>,
) {
    egui::Window::new("Login").show(egui_context.ctx_mut(), |ui| {
        ui.text_edit_singleline(&mut login_state.name);
        ui.text_edit_singleline(&mut login_state.room);

        if ui.button("Login").clicked() {
            game_state.set(GameState::Connecting).unwrap_or_default();
        }
    });
}

fn network_debug_ui(
    mut egui_context: ResMut<EguiContext>,
    peer_to_negotation: Res<crate::network::PeerToNegotiation>,
) {
    egui::Window::new("Network Debug")
        .show(egui_context.ctx_mut(), |ui| {
            egui::Grid::new("debug_grid")
                .num_columns(2)
                .spacing([40.0, 40.0])
                .striped(true)
                .show(ui, |ui| {
                    for (peer_id, negotiation) in peer_to_negotation.iter() {
                        ui.label("Peer: ");
                        ui.label(peer_id);
                        ui.end_row();

                        ui.label("sequence out: ");
                        ui.label(negotiation.seq_out.to_string());
                        ui.end_row();

                        ui.label("last seen: ");
                        ui.label(negotiation.last_seen.to_string());
                        ui.end_row();
                    }
                });
        });

}
