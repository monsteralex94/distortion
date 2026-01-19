use clap::Parser;
use hound::{WavReader, WavWriter};
use std::error::Error;

#[derive(Parser)]
struct Args {
    input: String,
    output: String,
    drive: f32,
    volume: f32,
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut reader = WavReader::open(args.input)?;

    let spec = reader.spec();

    let mut samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / i16::MAX as f32)
        .collect();

    for s in &mut samples {
        *s = (*s * args.drive).tanh() * args.volume;
    }

    let mut writer = WavWriter::create(args.output, spec)?;

    for s in samples {
        let s_i16 = (s * i16::MAX as f32).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
        writer.write_sample(s_i16)?;
    }

    writer.finalize()?;

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
