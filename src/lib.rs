#[macro_use]
extern crate vst;
extern crate vst_gui;

use std::f32::consts::PI;
use std::sync::{Arc, Mutex};

use vst::buffer::AudioBuffer;
use vst::editor::Editor;
use vst::plugin::{Category, Plugin, Info};


fn inline_script(s: &str) -> String {
	format!(r#"<script type="text/javascript">{}</script>"#, s)
}

fn get_html() -> String {
    format!(r#"
        <!doctype html>
        <html>
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
            </head>
            <body>
                <div id="app"></div>
                <!--[if lt IE 9]>
                <div class="ie-upgrade-container">
                    <p class="ie-upgrade-message">Please, upgrade Internet Explorer to continue using this software.</p>
                    <a class="ie-upgrade-link" target="_blank" href="https://www.microsoft.com/en-us/download/internet-explorer.aspx">Upgrade</a>
                </div>
                <![endif]-->
                <!--[if gte IE 9 | !IE ]> <!-->
                {scripts}
                <![endif]-->
            </body>
        </html>
        "#,
        scripts = inline_script(include_str!("bundle.js"))
    )
}

struct Oscillator {
    pub frequency: f32,
    pub waveform: f32,
    pub phase: f32,
    pub amplitude: f32,
}

fn create_javascript_callback(
    oscillator: Arc<Mutex<Oscillator>>) -> vst_gui::JavascriptCallback
{
    Box::new(move |message: String| {
        let mut tokens = message.split_whitespace();

        let command = tokens.next().unwrap_or("");
        let argument = tokens.next().unwrap_or("").parse::<f32>();

        let mut locked_oscillator = oscillator.lock().unwrap();

        String::new()
    })
}

struct ExampleSynth {
    sample_rate: f32,
    oscillator: Arc<Mutex<Oscillator>>,
}

impl Default for ExampleSynth {
    fn default() -> ExampleSynth {
        let oscillator = Arc::new(Mutex::new(
            Oscillator {
                frequency: 440.0,
                waveform: 0.0,
                phase: 0.0,
                amplitude: 0.1,
            }
        ));

        ExampleSynth {
            sample_rate: 44100.0,
            oscillator: oscillator.clone(),
        }
    }
}

impl Plugin for ExampleSynth {
    fn get_info(&self) -> Info {
        Info {
            name: "Example Synth".to_string(),
            vendor: "rust-vst-gui".to_string(),
            unique_id: 9614,
            category: Category::Synth,
            inputs: 2,
            outputs: 2,
            parameters: 0,
            initial_delay: 0,
            f64_precision: false,
            ..Info::default()
        }
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate as f32;
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {

    }

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        let gui = vst_gui::new_plugin_gui(
            String::from(get_html()),
            create_javascript_callback(self.oscillator.clone()),
            Some((480, 320)));
        Some(Box::new(gui))
    }
}

plugin_main!(ExampleSynth);
