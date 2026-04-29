mod gateway_poller;

pub use gateway_poller::{cached_status, GatewayPoller};

use pathfinder_color::ColorU;
use warp_core::ui::icons::Icon as WarpIcon;
use warp_core::ui::theme::Fill;
use warpui::elements::{ChildAnchor, ConstrainedBox, MouseStateHandle, OffsetPositioning, ParentAnchor, ParentOffsetBounds, ParentElement, Stack};
use warpui::prelude::vec2f;
use warpui::prelude::Hoverable;
use warpui::{AppContext, Element};

use crate::appearance::Appearance;

/// Render the KiloCore gateway health indicator for the pane header.
///
/// Shows a globe icon colored green (reachable) or muted (unreachable).
/// On hover, a tooltip shows the full label with service count and tool count.
pub fn render_gateway_indicator(
    mouse_state: MouseStateHandle,
    app: &AppContext,
) -> Box<dyn Element> {
    let appearance = Appearance::as_ref(app);
    let theme = appearance.theme();
    let font_size = appearance.ui_font_size();
    let ui_builder = appearance.ui_builder().clone();
    let status = cached_status(app);

    let icon_color = if status.reachable {
        // Accent green — matches KiloCore Dark theme #30a46c
        Fill::Solid(ColorU::new(48, 164, 108, 255))
    } else {
        theme.sub_text_color(theme.background())
    };
    let label = status.label();

    Hoverable::new(mouse_state, move |state| {
        let mut stack = Stack::new().with_child(
            ConstrainedBox::new(WarpIcon::Globe.to_warpui_icon(icon_color).finish())
                .with_height(font_size)
                .with_width(font_size)
                .finish(),
        );
        if state.is_hovered() {
            let tooltip = ui_builder
                .tool_tip(label.clone())
                .build()
                .finish();
            stack.add_positioned_overlay_child(
                tooltip,
                OffsetPositioning::offset_from_parent(
                    vec2f(0., 3.),
                    ParentOffsetBounds::WindowByPosition,
                    ParentAnchor::BottomMiddle,
                    ChildAnchor::TopMiddle,
                ),
            );
        }
        stack.finish()
    })
    .finish()
}

/// Register the GatewayPoller singleton in the app context.
pub fn register(app: &mut impl warpui::AddSingletonModel) {
    app.add_singleton_model(GatewayPoller::new);
}
