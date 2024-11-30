/// Module for creating device class u32s by name.

// Major and minor device class traits

pub trait MajorDeviceClass {
    fn major_device_class() -> u32;
}

pub trait MinorDeviceClass {
    fn minor_device_class(&self) -> u32;
}

/// Device class consisting of a major and minor class
pub trait DeviceClass {
    fn device_class(&self) -> u32;
}

/// Automatic implementation of DeviceClass for any types that implement both MajorDeviceClass and
/// MinorDeviceClass.
impl <T: MajorDeviceClass + MinorDeviceClass> DeviceClass for T {
    /// Convert this class to a DeviceClass by joining the major and minor device classes.
    fn device_class(&self) -> u32 {
        Self::major_device_class() | self.minor_device_class()
    }
}

// Major service class

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MajorServiceClass {
    pub limited_discoverable_mode: bool,
    pub le_audio: bool,
    pub positioning: bool,
    pub networking: bool,
    pub rendering: bool,
    pub capturing: bool,
    pub object_transfer: bool,
    pub audio: bool,
    pub telephony: bool,
    pub information: bool,
}

impl MajorServiceClass {
    /// Construct a new MajorServiceClass without any capabilities.
    pub const fn empty() -> Self {
        Self {
            limited_discoverable_mode: false,
            le_audio: false,
            positioning: false,
            networking: false,
            rendering: false,
            capturing: false,
            object_transfer: false,
            audio: false,
            telephony: false,
            information: false,
        }
    }
    
    /// Convert this MajorServiceClass into a masked u32.
    pub fn major_service_class(&self) -> u32 {
        0
            | if self.limited_discoverable_mode { 1 << 13 } else { 0 }
            | if self.le_audio { 1 << 14 } else { 0 }
            | if self.positioning { 1 << 16 } else { 0 }
            | if self.networking { 1 << 17 } else { 0 }
            | if self.rendering { 1 << 18 } else { 0 }
            | if self.capturing { 1 << 19 } else { 0 }
            | if self.object_transfer { 1 << 20 } else { 0 }
            | if self.audio { 1 << 21 } else { 0 }
            | if self.telephony { 1 << 22 } else { 0 }
            | if self.information { 1 << 23 } else { 0 }
    }
}


/// Create a u32 representing a class of device from a service class and device class.
pub fn make_class_of_device<C: DeviceClass>(
    major_service_class: MajorServiceClass,
    device_class: C,
) -> u32 {
    major_service_class.major_service_class() | device_class.device_class()
}


// Miscellaneous class

pub struct Miscellaneous {
    pub minor_device_class: u32,
}

impl MajorDeviceClass for Miscellaneous {
    /// Returns the major class for all Miscellaneous classes.
    fn major_device_class() -> u32 {
        0x0000
    }
}

impl MinorDeviceClass for Miscellaneous {
    /// Convert this Miscellaneous class into a minor device class.
    fn minor_device_class(&self) -> u32 {
        self.minor_device_class
    }
}

// Computer class

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Computer {
    #[default]
    Uncategorized,
    DesktopWorkstation,
    ServerClassComputer,
    Laptop,
    HandheldPcPda,
    PalmSizedPcPda,
    WearableComputer,
    Tablet,
}

impl MajorDeviceClass for Computer {
    /// Returns the major class for all Computer classes.
    fn major_device_class() -> u32 {
        0x0100
    }
}

impl MinorDeviceClass for Computer {
    /// Convert this Computer class into a minor device class.
    fn minor_device_class(&self) -> u32 {
        match self {
            Self::Uncategorized => 0x00,
            Self::DesktopWorkstation => 0x04,
            Self::ServerClassComputer => 0x08,
            Self::Laptop => 0x0C,
            Self::HandheldPcPda => 0x10,
            Self::PalmSizedPcPda => 0x14,
            Self::WearableComputer => 0x18,
            Self::Tablet => 0x1C,
        }
    }
}

// Phone

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Phone {
    #[default]
    Uncategorized,
    Cellular,
    Cordless,
    Smartphone,
    WiredModemOrVoiceGateway,
    CommonIsdnAccess,
}

impl MajorDeviceClass for Phone {
    /// Returns the major class for all Phone classes.
    fn major_device_class() -> u32 {
        0x0200
    }
}

impl MinorDeviceClass for Phone {
    /// Convert this Phone class into a minor device class.
    fn minor_device_class(&self) -> u32 {
        match self {
            Self::Uncategorized => 0x00,
            Self::Cellular => 0x04,
            Self::Cordless => 0x08,
            Self::Smartphone => 0x0C,
            Self::WiredModemOrVoiceGateway => 0x10,
            Self::CommonIsdnAccess => 0x14,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LanNetworkAccessPoint {
    #[default]
    FullyAvailable,
    Utilized1To17Percent,
    Utilized17To33Percent,
    Utilized33To50Percent,
    Utilized50To67Percent,
    Utilized67To83Percent,
    Utilized83To99Percent,
    NoServiceAvailable,
}

impl MajorDeviceClass for LanNetworkAccessPoint {
    /// Returns the major class for all LanNetworkAccessPoint classes.
    fn major_device_class() -> u32 {
        0x0300
    }
}

impl MinorDeviceClass for LanNetworkAccessPoint {
    /// Convert this LanNetworkAccessPoint class into a minor device class.
    fn minor_device_class(&self) -> u32 {
        match self {
            Self::FullyAvailable => 0x00,
            Self::Utilized1To17Percent => 0x20,
            Self::Utilized17To33Percent => 0x40,
            Self::Utilized33To50Percent => 0x60,
            Self::Utilized50To67Percent => 0x80,
            Self::Utilized67To83Percent => 0xA0,
            Self::Utilized83To99Percent => 0xC0,
            Self::NoServiceAvailable => 0xE0,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AudioVideo {
    #[default]
    Uncategorized,
    WearableHeadsetDevice,
    HandsFreeDevice,

    Microphone,
    Loudspeaker,
    Headphones,
    PortableAudio,
    CarAudio,
    SetTopBox,
    HiFiAudioDevice,
    Vcr,
    VideoCamera,
    Camcorder,
    VideoMonitor,
    VideoDisplayAndLoudspeaker,
    VideoConferencing,

    GamingToy,
}

impl MajorDeviceClass for AudioVideo {
    /// Returns the major class for all AudioVideo classes.
    fn major_device_class() -> u32 {
        0x0500
    }
}

impl MinorDeviceClass for AudioVideo {
    /// Convert this AudioVideo class into a minor device class.
    fn minor_device_class(&self) -> u32 {
        match self {
            Self::Uncategorized => 0x00,
            Self::WearableHeadsetDevice => 0x04,
            Self::HandsFreeDevice => 0x08,

            Self::Microphone => 0x10,
            Self::Loudspeaker => 0x14,
            Self::Headphones => 0x18,
            Self::PortableAudio => 0x1C,
            Self::CarAudio => 0x20,
            Self::SetTopBox => 0x24,
            Self::HiFiAudioDevice => 0x28,
            Self::Vcr => 0x2C,
            Self::VideoCamera => 0x30,
            Self::Camcorder => 0x34,
            Self::VideoMonitor => 0x38,
            Self::VideoDisplayAndLoudspeaker => 0x3C,
            Self::VideoConferencing => 0x40,

            Self::GamingToy => 0x48,
        }
    }
}

// Peripheral class

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PeripheralUpper {
    #[default]
    Uncategorized,
    Keyboard,
    PointingDevice,
    ComboKeyboardPointingDevice,
}

impl PeripheralUpper {
    pub const fn code(&self) -> u32 {
        match self {
            Self::Uncategorized => 0x00,
            Self::Keyboard => 0x40,
            Self::PointingDevice => 0x80,
            Self::ComboKeyboardPointingDevice => 0xC0,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PeripheralLower {
    #[default]
    Uncategorized,
    Joystick,
    Gamepad,
    RemoteControl,
    SensingDevice,
    DigitizerTablet,
    CardReader,
    DigitalPen,
    HandheldScanner,
    HandheldGesturalInputDevice,
}

impl PeripheralLower {
    pub const fn code(&self) -> u32 {
        match self {
            Self::Uncategorized => 0x00,
            Self::Joystick => 0x04,
            Self::Gamepad => 0x08,
            Self::RemoteControl => 0x0C,
            Self::SensingDevice => 0x10,
            Self::DigitizerTablet => 0x14,
            Self::CardReader => 0x18,
            Self::DigitalPen => 0x1C,
            Self::HandheldScanner => 0x20,
            Self::HandheldGesturalInputDevice => 0x24,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Peripheral {
    pub upper: PeripheralUpper,
    pub lower: PeripheralLower,
}

impl Peripheral {
    /// Construct a new Peripheral from an upper and lower part.
    pub fn new(upper: PeripheralUpper, lower: PeripheralLower) -> Self {
        Self { upper, lower }
    }
}

impl MajorDeviceClass for Peripheral {
    /// Returns the major device class of all Peripheral classes.
    fn major_device_class() -> u32 {
        0x0500
    }
}

impl MinorDeviceClass for Peripheral {
    /// Convert this Peripheral class into a minor device class.
    fn minor_device_class(&self) -> u32 {
        self.upper.code() | self.lower.code()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Imaging {
    pub display: bool,
    pub camera: bool,
    pub scanner: bool,
    pub printer: bool,
}

impl MajorDeviceClass for Imaging {
    fn major_device_class() -> u32 {
        0x0600
    }
}

impl MinorDeviceClass for Imaging {
    /// Convert this Imaging class into a minor device class.
    fn minor_device_class(&self) -> u32 {
        0
            | if self.display { 1 << 4 } else { 0 }
            | if self.camera { 1 << 5 } else { 0 }
            | if self.scanner { 1 << 6 } else { 0 }
            | if self.printer { 1 << 7 } else { 0 }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Wearable {
    Wristwatch,
    Pager,
    Jacket,
    Helmet,
    Glasses,
    Pin,
}

impl MajorDeviceClass for Wearable {
    /// Returns the major class for all Wearable classes.
    fn major_device_class() -> u32 {
        0x0700
    }
}

impl MinorDeviceClass for Wearable {
    /// Convert this Wearable into a minor device class.
    fn minor_device_class(&self) -> u32 {
        match self {
            Self::Wristwatch => 0x04,
            Self::Pager => 0x08,
            Self::Jacket => 0x0C,
            Self::Helmet => 0x10,
            Self::Glasses => 0x14,
            Self::Pin => 0x18,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Toy {
    Robot,
    Vehicle,
    DollActionFigure,
    Controller,
    Game,
}

impl MajorDeviceClass for Toy {
    /// Returns the major class of all Toy classes.
    fn major_device_class() -> u32 {
        0x0800
    }
}

impl MinorDeviceClass for Toy {
    /// Convert this Toy class into a minor device class.
    fn minor_device_class(&self) -> u32 {
        match self {
            Self::Robot => 0x04,
            Self::Vehicle => 0x08,
            Self::DollActionFigure => 0x0C,
            Self::Controller => 0x10,
            Self::Game => 0x14,
        }
    }
}

// Health

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Health {
    #[default]
    Undefined,
    BloodPressureMonitor,
    Thermometer,
    WeighingScale,
    GlucoseMeter,
    PulseOximeter,
    HeartPulseRateMonitor,
    HealthDataDisplay,
    StepCounter,
    BodyCompositionAnalyzer,
    PeakFlowMonitor,
    MedicationMonitor,
    KneeProsthesis,
    AnkleProsthesis,
    GenericHealthManager,
    PersonalMobilityDevice,
}

impl MajorDeviceClass for Health {
    /// Returns the major class of all Health classes.
    fn major_device_class() -> u32 {
        0x0900
    }
}

impl MinorDeviceClass for Health {
    /// Convert this Health class into a minor device class.
    fn minor_device_class(&self) -> u32 {
        match self {
            Self::Undefined => 0x00,
            Self::BloodPressureMonitor => 0x04,
            Self::Thermometer => 0x08,
            Self::WeighingScale => 0x0C,
            Self::GlucoseMeter => 0x10,
            Self::PulseOximeter => 0x14,
            Self::HeartPulseRateMonitor => 0x18,
            Self::HealthDataDisplay => 0x1C,
            Self::StepCounter => 0x20,
            Self::BodyCompositionAnalyzer => 0x24,
            Self::PeakFlowMonitor => 0x28,
            Self::MedicationMonitor => 0x2C,
            Self::KneeProsthesis => 0x30,
            Self::AnkleProsthesis => 0x34,
            Self::GenericHealthManager => 0x38,
            Self::PersonalMobilityDevice => 0x3C,
        }
    }
}

// Uncategorizrd

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Uncategorized {
    pub minor_device_class: u32,
}

impl MajorDeviceClass for Uncategorized {
    /// Returns the major class for all Uncategorized classes.
    fn major_device_class() -> u32 {
        0x1F00
    }
}

impl MinorDeviceClass for Uncategorized {
    /// Convert this Uncategorized class into a minor device class.
    fn minor_device_class(&self) -> u32 {
        self.minor_device_class
    }
}
