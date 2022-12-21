use gtk::{Builder, Button, GestureClick, Label};
use gtk::prelude::*;
use std::sync::Arc;
use crate::pads::audio::sample::Sample;
use crate::pads::audio::sample_player::SamplePlayer;


// TODO: Implement a custom widget instead of Label.

pub struct Pad {
    id: i32,
    sample: Option<Sample>,
    widget: Label, // Temporarily using Label here.
    sample_player: Arc<SamplePlayer>,
}

impl Pad {
    pub fn new(id: i32, widget: Label, sample: Option<Sample>, sample_player: Arc<SamplePlayer>) -> Arc<Pad> {
        let gesture_click = GestureClick::new();
        let pad = Arc::new(Pad {id: id, widget: widget, sample: sample, sample_player: sample_player});
        let cloned_pad = Arc::clone(&pad);
        gesture_click.connect_pressed(move |_gesture, _n_press, _x, _y| cloned_pad.on_click());
        pad.widget.add_controller(&gesture_click);

        pad
    }

    fn on_click(&self) {
        match &self.sample {
            Some(s) => {
                let _ = &self.sample_player.play_sample(s);
            },
            None => (),
        }
    }
}

pub fn get_label_by_id(id: i32, builder: &Builder) -> Option<Label> {
    builder.object(format!("pad-{}", id).as_str())
}
