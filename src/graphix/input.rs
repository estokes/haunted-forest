use bevy::winit::RawWinitWindowEvent;
use graphix_package_gui::convert;
use iced_core::mouse;
use winit::keyboard::ModifiersState;

use super::surface::IcedSurface;

/// Tracks modifier key state and routes input events to iced surfaces.
pub struct InputRouter {
    modifiers: ModifiersState,
}

impl InputRouter {
    pub fn new() -> Self {
        Self { modifiers: ModifiersState::default() }
    }

    /// Process raw winit events and push them to the appropriate surface.
    /// For now, all input goes to the overlay surface (first surface found).
    pub fn route_events(
        &mut self,
        raw_events: &[RawWinitWindowEvent],
        surfaces: &mut [&mut IcedSurface],
    ) {
        for raw in raw_events {
            if let winit::event::WindowEvent::ModifiersChanged(m) = &raw.event {
                self.modifiers = m.state();
            }
            // Convert winit event to iced events
            let mut iced_events = convert::window_event(&raw.event, 1.0, self.modifiers);
            for ev in iced_events.drain(..) {
                // Track cursor position
                if let iced_core::Event::Mouse(mouse::Event::CursorMoved {
                    position,
                }) = &ev
                {
                    for surface in surfaces.iter_mut() {
                        surface.cursor_position = *position;
                    }
                }
                // Route to all surfaces (overlay gets priority in practice)
                for surface in surfaces.iter_mut() {
                    surface.push_event(ev.clone());
                }
            }
        }
    }
}
