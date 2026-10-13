//! Typing guard for a Right Shift trigger.
//!
//! When a binding is the bare `shift_right` modifier, a recording starts the
//! moment the key goes down. If another key is pressed while it is held, the
//! user was typing a capital letter, not dictating: cancel the recording
//! quietly so nothing is pasted.

use handy_keys::{KeyboardListener, Modifiers};
use log::{debug, error};
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use tauri::{AppHandle, Manager};

use crate::managers::audio::AudioRecordingManager;
use crate::settings::get_settings;

/// Spawn the watcher thread. Runs for the life of the app.
pub fn spawn_guard(app: AppHandle) {
    thread::spawn(move || {
        let listener = match KeyboardListener::new() {
            Ok(l) => l,
            Err(e) => {
                error!("Right Shift guard could not start: {}", e);
                return;
            }
        };

        loop {
            let Some(event) = listener.try_recv() else {
                thread::sleep(Duration::from_millis(5));
                continue;
            };

            // Modifier changes carry no key; only real keys mean "typing".
            if !event.is_key_down || event.key.is_none() {
                continue;
            }
            if !event.modifiers.contains(Modifiers::SHIFT_RIGHT) {
                continue;
            }

            // Only touch an active recording, never one that is already processing.
            if !app.state::<Arc<AudioRecordingManager>>().is_recording() {
                continue;
            }

            let uses_right_shift = get_settings(&app)
                .bindings
                .values()
                .any(|b| b.current_binding.trim() == "shift_right");
            if uses_right_shift {
                debug!("Key pressed while Right Shift held; cancelling dictation");
                crate::utils::cancel_current_operation(&app);
            }
        }
    });
}
