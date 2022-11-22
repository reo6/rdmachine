use gtk::prelude::*;
use gtk::{
    Application,
    ApplicationWindow,
    Builder,
};
use crate::pads::Pad;
use crate::pads::get_btn_by_id;
use crate::pads::sample_player::SamplePlayer;
use crate::pads::sample::Sample;
use std::sync::Arc;
use std::fs::File;

mod pads;

const APP_ID: &str = "org.reo.rdmachine";
const UI_CONTENT: &str = include_str!("ui/rdmachine.ui");
const TEST_SAMPLE_PATH: &str = "/home/reo/Downloads/Cymatics - Ultimate Lofi Collection/Cymatics - Lofi Drums Collection/Drums - One Shots/Snaps/Cymatics - Lofi Snap 4.wav";


fn main() {
    let app = Application::new(
        Some(APP_ID),
        Default::default(),
    );

    let sample_player = Arc::new(SamplePlayer::new());
    app.connect_activate(move |app| build_ui(app, Arc::clone(&sample_player)));

    app.run();
}

fn generate_pads(builder: &Builder, n: i32, sample_player: Arc<SamplePlayer>) -> Vec<Arc<Pad>> {
    let mut padvec: Vec<Arc<Pad>> = vec![];

    for i in 1..(n+1) {
        padvec.push(Pad::new(
            i,
            get_btn_by_id(i, builder).expect("Couldn't get pad."),
            Some(Sample::new(File::open(TEST_SAMPLE_PATH).unwrap())),
            Arc::clone(&sample_player),
        ));
    }

    padvec
}

fn build_ui(app: &Application, sample_player: Arc<SamplePlayer>) {
    let builder = Builder::from_string(UI_CONTENT);
    let pads = generate_pads(&builder, 16, Arc::clone(&sample_player));

    let window: ApplicationWindow = builder.object("main-window").expect("Couldn't get window.");
    window.set_application(Some(app));
    window.show();
}


