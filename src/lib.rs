pub mod session;

#[cfg(test)]
mod tests {
    use session::Category;

    use super::*;

    #[test]
    fn can_set_category() {
        let result = std::panic::catch_unwind(|| {
            let shared_instance = session::AVAudioSession::shared_instance();
            shared_instance.set_category(Category::ambient());
        });

        assert!(result.is_ok());
    }
}
