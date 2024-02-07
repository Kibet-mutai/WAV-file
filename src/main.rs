use std::{f64::consts::PI, fs::File, io::{Write, self}};

trait Play {
    // add code here
    // TODO
}

#[derive(Debug)]
struct SineOscillator {
    frequency: f64,
    amplitude: f64,
    angle: f64,
}

impl SineOscillator {
    fn new(frequency: f64, amplitude: f64) -> Self{
        SineOscillator { frequency: frequency, amplitude: amplitude, angle: 0.0 }
    }
    fn process(&mut self) -> f64 {
        let sample_rate = 44100.0;
        let sample = self.amplitude * self.angle.sin();
        self.angle += 2.0 * PI * self.frequency/sample_rate;
        sample
    }
}

fn write_to_file(mut file: File,  value: &u8) -> io::Result<()> {
    let _ = file.write_all(&value.to_be_bytes());
    Ok(())
}
fn main() {
    let sample_rate = 44100.0;
    let bit_depth = 16.0;
    let mut sine_oscillator = SineOscillator::new(440.0, 0.5);
    let mut max_amplitude = (2i64.pow(16) - 1) - 1;
    let mut audio_file = File::create("waveform.wav").unwrap();
    // Header chcunk
    let _ = audio_file.write_all(b"RIFF");
    let _ = audio_file.write_all(b"----");
    let _ = audio_file.write_all(b"WAVE");

    // Format chunk
    let compression_code: u8 = 1;
    let div = (sample_rate / bit_depth) as u16;
    let bit_div = (bit_depth / 8.0) as u16;
    let sp = sample_rate as u16;
    let bd = bit_depth as u16;
    let _ = audio_file.write_all(b"fmt ");
    let _ = audio_file.write_all(&compression_code.to_be_bytes());
    let _ = audio_file.write_all(&compression_code.to_be_bytes());
    let _ = audio_file.write_all(&sp.to_be_bytes());
    let _ = audio_file.write_all(&div.to_be_bytes());
    let _ = audio_file.write_all(&bit_div.to_be_bytes());
    let _ = audio_file.write_all(&bd.to_be_bytes());


    // Data chunk
    //
    let _ = audio_file.write_all(b"data");
    let _ = audio_file.write_all(b"----");

    //TODO
    //-- find the size pf the chunk instead of hypens

    for _ in 0..sample_rate as i64 * 2 {
        let mut sample = sine_oscillator.process();
        let scaled_sample = (sample * max_amplitude as f64) as u16;
        let sample_bytes = scaled_sample.to_be_bytes();

        let _ = audio_file.write_all(&sample_bytes);
            //write!(audio_file, "{}\n", &sine_oscillator.process());
    }
   // audio_file.flush();
    //Ok(())
}
