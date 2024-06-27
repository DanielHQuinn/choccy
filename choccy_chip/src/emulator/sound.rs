#[cfg(feature = "sound")]
/// This module contains the sound struct used to play the audio for the Chip-8 emulator.
pub mod Audio {
    use std::time::Duration;
    use std::fmt;
    use rodio::{OutputStreamHandle, OutputStream, Sink};
    use rodio::source::{SineWave, Source};

    /// The `Sound` struct is used to play audio in the CHIP-8 emulator.
    pub struct Audio {
        sink: Sink,
        stream_handle: OutputStreamHandle,
        stream: OutputStream,
    }

    impl Audio {
        #[must_use]
        #[allow(clippy::new_without_default)]
        /// Creates a new instance of the Sound struct.
        ///
        /// # Panics
        ///
        /// This function panics if it fails to get the default output stream or create the sink.
        pub fn new() -> Self {
            let (stream, stream_handle) = OutputStream::try_default().expect("Failed to get default output stream");
            let sink = Sink::try_new(&stream_handle).expect("Failed to create sink");
            sink.pause();
        
            Self { sink, stream_handle, stream }
        }

        /// Plays the sound.
        pub fn play(&self) {
            // Play a 440Hz sine wave for 0.25 seconds at 20% volume. 440 Hz is the standard tuning frequency.
            let source = SineWave::new(440.0).take_duration(Duration::from_secs_f32(0.25)).amplify(0.20);
            self.sink.append(source);
            self.sink.play();
        }
    }

    impl fmt::Debug for Audio {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Sound")
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[cfg(target_os = "macos")]
        #[test]
        fn test_sound() {
            let sound = Audio::new();

            sound.play();
            std::thread::sleep(std::time::Duration::from_secs(1));
            sound.play();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
