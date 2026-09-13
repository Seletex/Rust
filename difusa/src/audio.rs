use std::io::Cursor;
use hound::{SampleFormat, WavSpec, WavWriter};

pub fn crear_wav_en_memoria(generar_muestra: impl Fn(f32) -> f32, duracion_secs: f32) -> Vec<u8> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut cursor = Cursor::new(Vec::new());
    {
        let mut writer = WavWriter::new(&mut cursor, spec).expect("Error creando WavWriter");
        let num_samples = (spec.sample_rate as f32 * duracion_secs) as usize;

        for i in 0..num_samples {
            let t = i as f32 / spec.sample_rate as f32;
            let sample_f32 = generar_muestra(t).clamp(-1.0, 1.0);
            let sample_i16 = (sample_f32 * 32767.0) as i16;
            writer.write_sample(sample_i16).unwrap();
        }
        writer.finalize().expect("Error al finalizar WAV");
    }

    cursor.into_inner()
}