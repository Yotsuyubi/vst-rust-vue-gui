#[macro_use]
extern crate vst;
extern crate vst_gui;

use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

use std::f32::consts::PI;
use std::sync::{Arc, Mutex};

use vst::buffer::AudioBuffer;
use vst::editor::Editor;
use vst::plugin::{Category, Plugin, Info};


fn vec_to_str(vec: Vec<f32>) -> String {
	let str_vec: Vec<String> = vec.iter().map(|x| x.to_string()).collect();
	str_vec.join(",")
}

fn float_vec_to_complex_vec(vec: Vec<f32>) -> Vec<Complex<f32>> {
	let mut conv_vec = vec![Complex::zero(); vec.len()];
	for (i, val) in conv_vec.iter_mut().enumerate() {
		*val = Complex::new(vec[i], 0.)
	}
	return conv_vec;
}

fn complex_vec_to_float_vec(vec: Vec<Complex<f32>>) -> Vec<f32> {
	let mut conv_vec = vec![0.; vec.len()];
	for (i, val) in conv_vec.iter_mut().enumerate() {
		*val = (vec[i].re.powf(2.0) + vec[i].im.powf(2.0)).sqrt()
	}
	return conv_vec;
}

fn inline_script(s: &str) -> String {
	format!(r#"<script type="text/javascript">{}</script>"#, s)
}

fn inline_style(s: &str) -> String {
	format!(r#"<style type="text/css">{}</style>"#, s)
}


fn get_html() -> String {
    format!(r#"
        <!doctype html>
        <html>
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
				{style}
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
		style = inline_style(include_str!("style.css")),
        scripts = inline_script(include_str!("bundle.js"))
    )
}

struct Spectrum {
    pub spectrum: Vec<f32>
}

fn create_javascript_callback(
    spectrum: Arc<Mutex<Spectrum>>) -> vst_gui::JavascriptCallback
{
    Box::new(move |message: String| {
        let mut tokens = message.split_whitespace();

        let command = tokens.next().unwrap_or("");
        let argument = tokens.next().unwrap_or("").parse::<f32>();

		let mut locked_spectrum = spectrum.lock().unwrap();

		match command {
			"getSpectrum" => {
				let spec = &locked_spectrum.spectrum;
				return vec_to_str(spec.to_vec());
			},
			_ => {}
		}

        String::new()
    })
}

struct ExampleSpectrum {
    sample_rate: f32,
    spectrum: Arc<Mutex<Spectrum>>,
}

impl Default for ExampleSpectrum {
    fn default() -> ExampleSpectrum {
        let spectrum = Arc::new(Mutex::new(
            Spectrum {
                spectrum: Vec::new()
            }
        ));

        ExampleSpectrum {
            sample_rate: 44100.0,
            spectrum: spectrum.clone(),
        }
    }
}

impl Plugin for ExampleSpectrum {
    fn get_info(&self) -> Info {
        Info {
            name: "Example".to_string(),
            vendor: "rust-vst-gui".to_string(),
            unique_id: 9614,
            category: Category::Unknown,
            inputs: 2,
            outputs: 2,
            parameters: 4,
            ..Info::default()
        }
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate as f32;
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {

		let (input_buffer, output_buffer) = buffer.split();
		let channel_l_buffer = input_buffer.get(0);
		let len = channel_l_buffer.to_vec().len();

		let mut input:  Vec<Complex<f32>> = float_vec_to_complex_vec(channel_l_buffer.to_vec());
		let mut output: Vec<Complex<f32>> = vec![Complex::zero(); len];

		let mut planner = FFTplanner::new(false);
		let fft = planner.plan_fft(len);
		fft.process(&mut input[..], &mut output[..]);

		// TODO: FFT
		let mut locked_spectrum = self.spectrum.lock().unwrap();
		// dummy
		locked_spectrum.spectrum = complex_vec_to_float_vec(output);
    }

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        let gui = vst_gui::new_plugin_gui(
            String::from(get_html()),
            create_javascript_callback(self.spectrum.clone()),
            Some((480, 320)));
        Some(Box::new(gui))
    }
}

plugin_main!(ExampleSpectrum);
