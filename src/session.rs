use avfaudio2_sys::{
    AVAudioSession as AVAudioSessionSys, AVAudioSessionCategory, AVAudioSessionCategoryAmbient,
    AVAudioSessionCategoryAudioProcessing, AVAudioSessionCategoryMultiRoute,
    AVAudioSessionCategoryPlayAndRecord, AVAudioSessionCategoryPlayback,
    AVAudioSessionCategoryRecord, AVAudioSessionCategorySoloAmbient, AVAudioSession_Activation,
    IAVAudioSession, NSError,
};

use objc::runtime::{NO, YES};

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

#[doc = "Constants that specify optional audio behaviors."]
#[doc = "https://developer.apple.com/documentation/avfaudio/avaudiosession/categoryoptions"]
pub struct CategoryOptions {}

impl CategoryOptions {
    #[doc = "An option that indicates whether audio from this session mixes with audio from active sessions in other audio apps."]
    pub const MIX_WITH_OTHERS: u64 = 1;

    #[doc = "An option that reduces the volume of other audio sessions while audio from this session plays."]
    pub const DUCK_OTHERS: u64 = 2;

    #[doc = "An option that determines whether Bluetooth hands-free devices appear as available input routes."]
    pub const ALLOW_BLUETOOTH: u64 = 4;

    #[doc = "An option that makes Bluetooth Hands-Free Profile (HFP) devices available for audio input."]
    pub const ALLOW_BLUETOOTH_HFP: u64 = 4;

    #[doc = "An option that determines whether audio from the session defaults to the built-in speaker instead of the receiver."]
    pub const DEFAULT_TO_SPEAKER: u64 = 8;

    #[doc = "An option that determines whether to pause spoken audio content from other sessions when your app plays its audio."]
    pub const INTERRUPT_SPOKEN_AUDIO_AND_MIX_WITH_OTHERS: u64 = 17;

    #[doc = "An option that determines whether you can stream audio from this session to Bluetooth devices that support the Advanced Audio Distribution Profile (A2DP)."]
    pub const ALLOW_BLUETOOTH_A2DP: u64 = 32;

    #[doc = "An option that determines whether you can stream audio from this session to AirPlay devices."]
    pub const ALLOW_AIRPLAY: u64 = 64;

    #[doc = "An option that indicates whether the system interrupts the audio session when it mutes the built-in microphone."]
    pub const OVERRIDE_MUTED_MICROPHONE_INTERRUPTION: u64 = 128;

    #[doc = "An option that indicates to enable high-quality audio for input and output routes."]
    pub const BLUETOOTH_HIGH_QUALITY_RECORDING: u64 = 524288;
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

    #[doc = "Sets the audio session’s category."]
    pub fn set_category(&self, category: Category) {
        if let Some(session) = self.session {
            #[allow(unused)]
            let mut error: *mut NSError = ::std::ptr::null_mut();

            unsafe { session.setCategory_error_(category.0, error) };
        }
    }

    #[doc = "Sets the audio session’s category and options."]
    pub fn set_category_with_options(&self, category: Category, options: u64) {
        if let Some(session) = self.session {
            #[allow(unused)]
            let mut error: *mut NSError = ::std::ptr::null_mut();

            unsafe { session.setCategory_withOptions_error_(category.0, options, error) };
        }
    }

    #[doc = "Activate audio session"]
    pub fn activate(&self) {
        if let Some(session) = self.session {
            #[allow(unused)]
            let mut error: *mut NSError = ::std::ptr::null_mut();

            unsafe { session.setActive_error_(YES, error) };
        }
    }

    #[doc = "Dectivate audio session"]
    pub fn deactivate(&self) {
        if let Some(session) = self.session {
            #[allow(unused)]
            let mut error: *mut NSError = ::std::ptr::null_mut();

            unsafe { session.setActive_error_(NO, error) };
        }
    }
}
