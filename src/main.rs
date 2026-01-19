use clap::Parser;
use hound;

#[derive(Parser)]
struct Args {
    input: String,
    output: String,
    drive: f32,
    volume: f32,
}

fn main() {
    let args = Args::parse();

    let mut reader = match hound::WavReader::open(args.input) {
        Ok(r) => r,
        Err(_) => return,
    };

    let spec = reader.spec();

    let mut samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / i16::MAX as f32)
        .collect();

    for s in &mut samples {
        *s = (*s * args.drive).tanh() * args.volume;
    }

    let mut writer = match hound::WavWriter::create(args.output, spec) {
        Ok(w) => w,
        Err(_) => return,
    };

    for s in samples {
        let s_i16 = (s * i16::MAX as f32).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
        match writer.write_sample(s_i16) {
            Ok(_) => (),
            Err(_) => return,
        };
    }

    match writer.finalize() {
        Ok(_) => (),
        Err(_) => return,
    };
}
