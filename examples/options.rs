use avfaudio::session::{AVAudioSession, Category, CategoryOptions};

pub fn main() {
    let shared_instance = AVAudioSession::shared_instance();
    shared_instance.set_category_with_options(
        Category::play_and_record(),
        CategoryOptions::MIX_WITH_OTHERS | CategoryOptions::DEFAULT_TO_SPEAKER
    );
}
