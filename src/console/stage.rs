use std::fmt::Display;
use std::str::FromStr;

use color_eyre::eyre::bail;
use color_eyre::Report;

use crate::packet::FixedString;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Mushroom,
    Cap,
    Cascade,
    Sand,
    Lake,
    Wooded,
    Cloud,
    Lost,
    Metro,
    Seaside,
    Snow,
    Luncheon,
    Ruined,
    Bowsers,
    Moon,
    DarkSide,
    DarkerSide,
}

impl Stage {
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            Stage::Mushroom => "mushroom",
            Stage::Cap => "cap",
            Stage::Cascade => "cascade",
            Stage::Sand => "sand",
            Stage::Lake => "lake",
            Stage::Wooded => "wooded",
            Stage::Cloud => "cloud",
            Stage::Lost => "lost",
            Stage::Metro => "metro",
            Stage::Seaside => "seaside",
            Stage::Snow => "snow",
            Stage::Luncheon => "luncheon",
            Stage::Ruined => "ruined",
            Stage::Bowsers => "bowsers",
            Stage::Moon => "moon",
            Stage::DarkSide => "darkside",
            Stage::DarkerSide => "darkerside",
        }
    }

    #[inline]
    pub fn stage_name(&self) -> &'static str {
        match self {
            Self::Mushroom => "PeachWorldHomeStage",
            Self::Cap => "CapWorldHomeStage",
            Self::Cascade => "WaterfallWorldHomeStage",
            Self::Sand => "SandWorldHomeStage",
            Self::Lake => "LakeWorldHomeStage",
            Self::Wooded => "ForestWorldHomeStage",
            Self::Cloud => "CloudWorldHomeStage",
            Self::Lost => "ClashWorldHomeStage",
            Self::Metro => "CityWorldHomeStage",
            Self::Seaside => "SeaWorldHomeStage",
            Self::Snow => "SnowWorldHomeStage",
            Self::Luncheon => "LavaWorldHomeStage",
            Self::Ruined => "BossRaidWorldHomeStage",
            Self::Bowsers => "SkyWorldHomeStage",
            Self::Moon => "MoonWorldHomeStage",
            Self::DarkSide => "Special1WorldHomeStage",
            Self::DarkerSide => "Special2WorldHomeStag",
        }
    }

    #[inline]
    pub fn stage_name_fixed(&self) -> FixedString<0x30> {
        let stage = self.stage_name();
        stage.parse().unwrap()
    }
}

impl FromStr for Stage {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        let stage = match lower.as_str() {
            "mushroom" | "mush" => Self::Mushroom,
            "cap" => Self::Cap,
            "cascade" => Self::Cascade,
            "sand" => Self::Sand,
            "lake" => Self::Lake,
            "wooded" => Self::Wooded,
            "cloud" => Self::Cloud,
            "lost" => Self::Lost,
            "metro" => Self::Metro,
            "seaside" | "sea" => Self::Seaside,
            "snow" => Self::Snow,
            "luncheon" | "lunch" => Self::Luncheon,
            "ruined" | "ruin" => Self::Ruined,
            "bowsers" | "bowser" => Self::Bowsers,
            "moon" => Self::Moon,
            "darkside" | "dark" => Self::DarkSide,
            "darkerside" | "darker" => Self::DarkerSide,

            _ => bail!("invalid stage"),
        };

        Ok(stage)
    }
}

impl Display for Stage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
