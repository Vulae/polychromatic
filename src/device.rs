use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{PolychromaticError, proc_bus_input_devices::query_input_devices};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Device {
    Keyboard(Keyboard),
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for Device {
    fn to_string(&self) -> String {
        match self {
            Device::Keyboard(keyboard) => keyboard.to_string(),
        }
    }
}

impl Device {
    pub(crate) fn icon(&self) -> &str {
        match self {
            Device::Keyboard(_) => "keyboard",
        }
    }

    pub(crate) fn matrix(&self) -> Option<(u32, u32)> {
        match self {
            Device::Keyboard(keyboard) => keyboard.matrix(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Keyboard {
    RazerBlackWidowUltimate2012,
    RazerBlackWidowStealthEdition,
    RazerAnansi,
    RazerDeathstalkerEssential,
    RazerBlackWidowUltimate2013,
    RazerBlackWidowStealth,
    RazerBlackWidowTournamentEdition2014,
    RazerDeathstalkerExpert,
    RazerBlackWidowChroma,
    RazerDeathStalkerChroma,
    RazerBlackWidowChromaTournamentEdition,
    RazerBlackWidowChromaOverwatch,
    RazerBlackWidowUltimate2016,
    RazerBlackWidowXChroma,
    RazerBlackWidowXUltimate,
    RazerBlackWidowXChromaTournamentEdition,
    RazerOrnataChroma,
    RazerOrnata,
    RazerBlackWidowChromaV2,
    RazerHuntsmanElite,
    RazerHuntsman,
    RazerBlackWidowElite,
    RazerCynosaChroma,
    RazerCynosaChromaPro,
    RazerBlackWidowLite,
    RazerBlackWidowEssential,
    RazerCynosaLite,
    RazerBlackWidow2019,
    RazerHuntsmanTournamentEdition,
    RazerBlackWidowV3,
    RazerHuntsmanMini,
    RazerBlackWidowV3MiniHyperspeedWired,
    RazerBlackWidowV3ProWired,
    RazerBlackWidowV3ProWireless,
    RazerOrnataV2,
    RazerCynosaV2,
    RazerHuntsmanV2Analog,
    RazerHuntsmanMiniJP,
    RazerBook13_2020,
    RazerHuntsmanV2Tenkeyless,
    RazerHuntsmanV2,
    RazerBlackWidowV3MiniHyperspeedWireless,
    RazerHuntsmanMiniAnalog,
    RazerBlackWidowV4,
    RazerBlackWidowV4Pro,
    RazerDeathStalkerV2ProWireless,
    RazerDeathStalkerV2ProWired,
    RazerBlackWidowV4X,
    RazerDeathStalkerV2,
    RazerDeathStalkerV2ProTKLWireless,
    RazerDeathStalkerV2ProTKLWired,
    RazerOrnataV3,
    RazerOrnataV3X,
    RazerOrnataV3Tenkeyless,
    RazerBlackWidowV3Tenkeyless,
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for Keyboard {
    fn to_string(&self) -> String {
        match self {
            Self::RazerBlackWidowUltimate2012 => "Razer BlackWidow Ultimate 2012",
            Self::RazerBlackWidowStealthEdition => "Razer BlackWidow Stealth Edition",
            Self::RazerAnansi => "Razer Anansi",
            Self::RazerDeathstalkerEssential => "Razer Deathstalker (Essential)",
            Self::RazerBlackWidowUltimate2013 => "Razer BlackWidow Ultimate 2013",
            Self::RazerBlackWidowStealth => "Razer BlackWidow Stealth",
            Self::RazerBlackWidowTournamentEdition2014 => {
                "Razer BlackWidow Tournament Edition 2014"
            }
            Self::RazerDeathstalkerExpert => "Razer Deathstalker Expert",
            Self::RazerBlackWidowChroma => "Razer BlackWidow Chroma",
            Self::RazerDeathStalkerChroma => "Razer DeathStalker Chroma",
            Self::RazerBlackWidowChromaTournamentEdition => {
                "Razer BlackWidow Chroma Tournament Edition"
            }
            Self::RazerBlackWidowChromaOverwatch => "Razer BlackWidow Chroma (Overwatch)",
            Self::RazerBlackWidowUltimate2016 => "Razer BlackWidow Ultimate 2016",
            Self::RazerBlackWidowXChroma => "Razer BlackWidow X Chroma",
            Self::RazerBlackWidowXUltimate => "Razer BlackWidow X Ultimate",
            Self::RazerBlackWidowXChromaTournamentEdition => {
                "Razer BlackWidow X Chroma Tournament Edition"
            }
            Self::RazerOrnataChroma => "Razer Ornata Chroma",
            Self::RazerOrnata => "Razer Ornata",
            Self::RazerBlackWidowChromaV2 => "Razer BlackWidow Chroma V2",
            Self::RazerHuntsmanElite => "Razer Huntsman Elite",
            Self::RazerHuntsman => "Razer Huntsman",
            Self::RazerBlackWidowElite => "Razer BlackWidow Elite",
            Self::RazerCynosaChroma => "Razer Cynosa Chroma",
            Self::RazerCynosaChromaPro => "Razer Cynosa Chroma Pro",
            Self::RazerBlackWidowLite => "Razer BlackWidow Lite",
            Self::RazerBlackWidowEssential => "Razer BlackWidow Essential",
            Self::RazerCynosaLite => "Razer Cynosa Lite",
            Self::RazerBlackWidow2019 => "Razer BlackWidow 2019",
            Self::RazerHuntsmanTournamentEdition => "Razer Huntsman Tournament Edition",
            Self::RazerBlackWidowV3 => "Razer BlackWidow V3",
            Self::RazerHuntsmanMini => "Razer Huntsman Mini",
            Self::RazerBlackWidowV3MiniHyperspeedWired => {
                "Razer BlackWidow V3 Mini Hyperspeed (Wired)"
            }
            Self::RazerBlackWidowV3ProWired => "Razer BlackWidow V3 Pro (Wired)",
            Self::RazerBlackWidowV3ProWireless => "Razer BlackWidow V3 Pro (Wireless)",
            Self::RazerOrnataV2 => "Razer Ornata V2",
            Self::RazerCynosaV2 => "Razer Cynosa V2",
            Self::RazerHuntsmanV2Analog => "Razer Huntsman V2 Analog",
            Self::RazerHuntsmanMiniJP => "Razer Huntsman Mini (JP)",
            Self::RazerBook13_2020 => "Razer Book 13 (2020)",
            Self::RazerHuntsmanV2Tenkeyless => "Razer Huntsman V2 Tenkeyless",
            Self::RazerHuntsmanV2 => "Razer Huntsman V2",
            Self::RazerBlackWidowV3MiniHyperspeedWireless => {
                "Razer BlackWidow V3 Mini Hyperspeed (Wireless)"
            }

            Self::RazerHuntsmanMiniAnalog => "Razer Huntsman Mini Analog",
            Self::RazerBlackWidowV4 => "Razer BlackWidow V4",
            Self::RazerBlackWidowV4Pro => "Razer BlackWidow V4 Pro",
            Self::RazerDeathStalkerV2ProWireless => "Razer DeathStalker V2 Pro (Wireless)",
            Self::RazerDeathStalkerV2ProWired => "Razer DeathStalker V2 Pro (Wired)",
            Self::RazerBlackWidowV4X => "Razer BlackWidow V4 X",
            Self::RazerDeathStalkerV2 => "Razer DeathStalker V2",
            Self::RazerDeathStalkerV2ProTKLWireless => "Razer DeathStalker V2 Pro TKL (Wireless)",
            Self::RazerDeathStalkerV2ProTKLWired => "Razer DeathStalker V2 Pro TKL (Wired)",
            Self::RazerOrnataV3 => "Razer Ornata V3",
            Self::RazerOrnataV3X => "Razer Ornata V3 X",
            Self::RazerOrnataV3Tenkeyless => "Razer Ornata V3 Tenkeyless",
            Self::RazerBlackWidowV3Tenkeyless => "Razer BlackWidow V3 Tenkeyless",
        }
        .to_owned()
    }
}

impl Keyboard {
    /// Automatically detect Razer keyboards
    pub fn detect() -> Result<Box<[Self]>, PolychromaticError> {
        Ok(query_input_devices()?
            .into_iter()
            .flat_map(|device| {
                if !device.name.contains("Razer") {
                    return None;
                }
                Some(Keyboard::iter().filter(move |kb| {
                    kb.device_id().iter().any(|(kb_id_vendor, kb_id_product)| {
                        device.id.vendor == *kb_id_vendor && device.id.product == *kb_id_product
                    })
                }))
            })
            .flatten()
            .collect())
    }

    /// Returns the first detected Razer keyboard
    pub fn detect_one() -> Result<Self, PolychromaticError> {
        Self::detect()?
            .into_iter()
            .next()
            .ok_or(PolychromaticError::NoRazerDevice)
    }

    pub(crate) fn device_id(&self) -> &[(u16, u16)] {
        match self {
            Self::RazerBlackWidowUltimate2012 => &[(0x1532, 0x010D)],
            Self::RazerBlackWidowStealthEdition => &[(0x1532, 0x010E)],
            Self::RazerAnansi => &[(0x1532, 0x010F)],
            Self::RazerDeathstalkerEssential => &[(0x1532, 0x0118)],
            Self::RazerBlackWidowUltimate2013 => &[(0x1532, 0x011A)],
            Self::RazerBlackWidowStealth => &[(0x1532, 0x011B)],
            Self::RazerBlackWidowTournamentEdition2014 => &[(0x1532, 0x011C)],
            Self::RazerDeathstalkerExpert => &[(0x1532, 0x0202)],
            Self::RazerBlackWidowChroma => &[(0x1532, 0x0203)],
            Self::RazerDeathStalkerChroma => &[(0x1532, 0x0204)],
            Self::RazerBlackWidowChromaTournamentEdition => &[(0x1532, 0x0209)],
            Self::RazerBlackWidowChromaOverwatch => &[(0x1532, 0x0211)],
            Self::RazerBlackWidowUltimate2016 => &[(0x1532, 0x0214)],
            Self::RazerBlackWidowXChroma => &[(0x1532, 0x0216)],
            Self::RazerBlackWidowXUltimate => &[(0x1532, 0x0217)],
            Self::RazerBlackWidowXChromaTournamentEdition => &[(0x1532, 0x021A)],
            Self::RazerOrnataChroma => &[(0x1532, 0x021E)],
            Self::RazerOrnata => &[(0x1532, 0x021F)],
            Self::RazerBlackWidowChromaV2 => &[(0x1532, 0x0221)],
            Self::RazerHuntsmanElite => &[(0x1532, 0x0226)],
            Self::RazerHuntsman => &[(0x1532, 0x0227)],
            Self::RazerBlackWidowElite => &[(0x1532, 0x0228)],
            Self::RazerCynosaChroma => &[(0x1532, 0x022A)],
            Self::RazerCynosaChromaPro => &[(0x1532, 0x022C)],
            Self::RazerBlackWidowLite => &[(0x1532, 0x0235)],
            Self::RazerBlackWidowEssential => &[(0x1532, 0x0237)],
            Self::RazerCynosaLite => &[(0x1532, 0x023F)],
            Self::RazerBlackWidow2019 => &[(0x1532, 0x0241)],
            Self::RazerHuntsmanTournamentEdition => &[(0x1532, 0x0243)],
            Self::RazerBlackWidowV3 => &[(0x1532, 0x024E)],
            Self::RazerHuntsmanMini => &[(0x1532, 0x0257)],
            Self::RazerBlackWidowV3MiniHyperspeedWired => &[(0x1532, 0x0258)],
            Self::RazerBlackWidowV3ProWired => &[(0x1532, 0x025A)],
            Self::RazerBlackWidowV3ProWireless => &[(0x1532, 0x025C)],
            Self::RazerOrnataV2 => &[(0x1532, 0x025D)],
            Self::RazerCynosaV2 => &[(0x1532, 0x025E)],
            Self::RazerHuntsmanV2Analog => &[(0x1532, 0x0266)],
            Self::RazerHuntsmanMiniJP => &[(0x1532, 0x0269)],
            Self::RazerBook13_2020 => &[(0x1532, 0x026A)],
            Self::RazerHuntsmanV2Tenkeyless => &[(0x1532, 0x026B)],
            Self::RazerHuntsmanV2 => &[(0x1532, 0x026C)],
            Self::RazerBlackWidowV3MiniHyperspeedWireless => &[(0x1532, 0x0271)],
            Self::RazerHuntsmanMiniAnalog => &[(0x1532, 0x0282)],
            Self::RazerBlackWidowV4 => &[(0x1532, 0x0287)],
            Self::RazerBlackWidowV4Pro => &[(0x1532, 0x028D)],
            Self::RazerDeathStalkerV2ProWireless => &[(0x1532, 0x0290)],
            Self::RazerDeathStalkerV2ProWired => &[(0x1532, 0x0292)],
            Self::RazerBlackWidowV4X => &[(0x1532, 0x0293)],
            Self::RazerDeathStalkerV2 => &[(0x1532, 0x0295)],
            Self::RazerDeathStalkerV2ProTKLWireless => &[(0x1532, 0x0296)],
            Self::RazerDeathStalkerV2ProTKLWired => &[(0x1532, 0x0298)],
            Self::RazerOrnataV3 => &[(0x1532, 0x02A1)],
            Self::RazerOrnataV3X => &[(0x1532, 0x02A2), (0x1532, 0x0294)],
            Self::RazerOrnataV3Tenkeyless => &[(0x1532, 0x02A3)],
            Self::RazerBlackWidowV3Tenkeyless => &[(0x1532, 0x0A24)],
        }
    }

    pub(crate) fn matrix(&self) -> Option<(u32, u32)> {
        match self {
            Self::RazerBlackWidowUltimate2012 => None,
            Self::RazerBlackWidowStealthEdition => None,
            Self::RazerAnansi => None,
            Self::RazerDeathstalkerEssential => None,
            Self::RazerBlackWidowUltimate2013 => None,
            Self::RazerBlackWidowStealth => None,
            Self::RazerBlackWidowTournamentEdition2014 => None,
            Self::RazerDeathstalkerExpert => None,
            Self::RazerBlackWidowChroma => Some((22, 6)),
            Self::RazerDeathStalkerChroma => Some((6, 1)),
            Self::RazerBlackWidowChromaTournamentEdition => Some((22, 6)),
            Self::RazerBlackWidowChromaOverwatch => Some((22, 6)),
            Self::RazerBlackWidowUltimate2016 => Some((22, 6)),
            Self::RazerBlackWidowXChroma => Some((22, 6)),
            Self::RazerBlackWidowXUltimate => Some((22, 6)),
            Self::RazerBlackWidowXChromaTournamentEdition => Some((22, 6)),
            Self::RazerOrnataChroma => Some((22, 6)),
            Self::RazerOrnata => Some((22, 6)),
            Self::RazerBlackWidowChromaV2 => Some((22, 6)),
            Self::RazerHuntsmanElite => Some((22, 9)),
            Self::RazerHuntsman => Some((22, 6)),
            Self::RazerBlackWidowElite => Some((22, 6)),
            Self::RazerCynosaChroma => Some((22, 6)),
            Self::RazerCynosaChromaPro => Some((22, 6)),
            Self::RazerBlackWidowLite => None,
            Self::RazerBlackWidowEssential => Some((22, 6)),
            Self::RazerCynosaLite => None,
            Self::RazerBlackWidow2019 => Some((22, 6)),
            Self::RazerHuntsmanTournamentEdition => Some((18, 6)),
            Self::RazerBlackWidowV3 => Some((22, 6)),
            Self::RazerHuntsmanMini => Some((15, 5)),
            Self::RazerBlackWidowV3MiniHyperspeedWired => Some((16, 5)),
            Self::RazerBlackWidowV3ProWired => Some((22, 6)),
            Self::RazerBlackWidowV3ProWireless => Some((22, 6)),
            Self::RazerOrnataV2 => Some((22, 6)),
            Self::RazerCynosaV2 => Some((22, 6)),
            Self::RazerHuntsmanV2Analog => Some((22, 8)),
            Self::RazerHuntsmanMiniJP => Some((15, 5)),
            Self::RazerBook13_2020 => None,
            Self::RazerHuntsmanV2Tenkeyless => Some((18, 6)),
            Self::RazerHuntsmanV2 => Some((22, 6)),
            Self::RazerBlackWidowV3MiniHyperspeedWireless => Some((16, 5)),
            Self::RazerHuntsmanMiniAnalog => Some((15, 5)),
            Self::RazerBlackWidowV4 => Some((23, 8)),
            Self::RazerBlackWidowV4Pro => Some((23, 8)),
            Self::RazerDeathStalkerV2ProWireless => Some((22, 6)),
            Self::RazerDeathStalkerV2ProWired => Some((22, 6)),
            Self::RazerBlackWidowV4X => Some((22, 6)),
            Self::RazerDeathStalkerV2 => Some((22, 6)),
            Self::RazerDeathStalkerV2ProTKLWireless => Some((17, 6)),
            Self::RazerDeathStalkerV2ProTKLWired => Some((17, 6)),
            Self::RazerOrnataV3 => Some((10, 1)),
            Self::RazerOrnataV3X => Some((1, 1)),
            Self::RazerOrnataV3Tenkeyless => Some((8, 1)),
            Self::RazerBlackWidowV3Tenkeyless => Some((18, 6)),
        }
    }
}
