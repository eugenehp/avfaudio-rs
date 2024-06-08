use avfaudio::session::{AVAudioSession, Category};

pub fn main() {
    let shared_instance = AVAudioSession::shared_instance();
    shared_instance.set_category(Category::ambient());
}
