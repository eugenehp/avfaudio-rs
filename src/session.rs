use avfaudio2_sys::{
    AVAudioSession as AVAudioSessionSys, AVAudioSessionCategory, AVAudioSessionCategoryAmbient,
    AVAudioSessionCategoryAudioProcessing, AVAudioSessionCategoryMultiRoute,
    AVAudioSessionCategoryPlayAndRecord, AVAudioSessionCategoryPlayback,
    AVAudioSessionCategoryRecord, AVAudioSessionCategorySoloAmbient, AVAudioSession_Activation,
    IAVAudioSession, NSError,
};

#[doc = "Audio session category identifiers."]
#[doc = "https://developer.apple.com/documentation/avfaudio/avaudiosessioncategory"]
pub struct Category(AVAudioSessionCategory);

impl Category {
    #[doc(alias = "AVAudioSessionCategoryAmbient")]
    #[doc = "Use this category for background sounds such as rain, car engine noise, etc.\nMixes with other music."]
    pub fn ambient() -> Category {
        unsafe { Category(AVAudioSessionCategoryAmbient) }
    }

    #[doc(alias = "AVAudioSessionCategorySoloAmbient")]
    #[doc = "Use this category for background sounds.  Other music will stop playing."]
    pub fn solo_ambient() -> Category {
        unsafe { Category(AVAudioSessionCategorySoloAmbient) }
    }

    #[doc(alias = "AVAudioSessionCategoryPlayback")]
    #[doc = "Use this category for music tracks."]
    pub fn playback() -> Category {
        unsafe { Category(AVAudioSessionCategoryPlayback) }
    }

    #[doc(alias = "AVAudioSessionCategoryRecord")]
    #[doc = "Use this category when recording audio."]
    pub fn record() -> Category {
        unsafe { Category(AVAudioSessionCategoryRecord) }
    }

    #[doc(alias = "AVAudioSessionCategoryPlayAndRecord")]
    #[doc = "Use this category when recording and playing back audio."]
    pub fn play_and_record() -> Category {
        unsafe { Category(AVAudioSessionCategoryPlayAndRecord) }
    }

    #[doc(alias = "AVAudioSessionCategoryAudioProcessing")]
    #[doc = "Use this category when using a hardware codec or signal processor while\nnot playing or recording audio."]
    pub fn audio_processing() -> Category {
        unsafe { Category(AVAudioSessionCategoryAudioProcessing) }
    }

    #[doc(alias = "AVAudioSessionCategoryMultiRoute")]
    #[doc = "Use this category to customize the usage of available audio accessories and built-in audio hardware.\nFor example, this category provides an application with the ability to use an available USB output\nand headphone output simultaneously for separate, distinct streams of audio data. Use of\nthis category by an application requires a more detailed knowledge of, and interaction with,\nthe capabilities of the available audio routes.  May be used for input, output, or both.\nNote that not all output types and output combinations are eligible for multi-route.  Input is limited\nto the last-in input port. Eligible inputs consist of the following:\nAVAudioSessionPortUSBAudio, AVAudioSessionPortHeadsetMic, and AVAudioSessionPortBuiltInMic.\nEligible outputs consist of the following:\nAVAudioSessionPortUSBAudio, AVAudioSessionPortLineOut, AVAudioSessionPortHeadphones, AVAudioSessionPortHDMI,\nand AVAudioSessionPortBuiltInSpeaker.\nNote that AVAudioSessionPortBuiltInSpeaker is only allowed to be used when there are no other eligible\noutputs connected."]
    pub fn multi_route() -> Category {
        unsafe { Category(AVAudioSessionCategoryMultiRoute) }
    }
}

#[doc = "An object that communicates to the system how you intend to use audio in your app."]
#[doc = "https://developer.apple.com/documentation/avfaudio/avaudiosession"]
#[derive(Debug)]
pub struct AVAudioSession {
    session: Option<AVAudioSessionSys>,
}

impl AVAudioSession {
    #[doc(alias = "shared_instance")]
    pub fn new() -> Self {
        AVAudioSession::shared_instance()
    }

    #[doc = "Returns the shared audio session instance."]
    pub fn shared_instance() -> Self {
        let session = unsafe { AVAudioSessionSys::sharedInstance() };
        AVAudioSession {
            session: Some(session),
        }
    }

    #[doc = "Sets the audio session’s category, mode, and options."]
    pub fn set_category(&self, category: Category) {
        if let Some(session) = self.session {
            #[allow(unused)]
            let mut error: *mut NSError = ::std::ptr::null_mut();

            unsafe { session.setCategory_error_(category.0, error) };
        }
    }

    #[doc = "activate audio session"]
    pub fn activate(&self) {
        if let Some(session) = self.session {
            #[allow(unused)]
            let mut error: *mut NSError = ::std::ptr::null_mut();

            unsafe { session.setActive_error_(true, error) };
        }
    }

    #[doc = "Dectivate audio session"]
    pub fn deactivate(&self) {
        if let Some(session) = self.session {
            #[allow(unused)]
            let mut error: *mut NSError = ::std::ptr::null_mut();

            unsafe { session.setActive_error_(false, error) };
        }
    }
}
