//! Melody registry for predefined JR East departure melodies.
//!
//! Melodies are named using the format: `{LINE}-{STATION}`
//!
//! Supported lines:
//! - JY: Yamanote Line (山手線)
//! - JK: Keihin-Tohoku Line (京浜東北線)
//! - JB: Sobu Line (総武線)
//! - JA: Saikyo Line (埼京線)
//! - JU: Ueno-Tokyo Line (上野東京ライン)
//! - NEX: Narita Express (成田エクスプレス)

use std::collections::HashMap;

/// Information about a predefined melody
#[derive(Debug, Clone)]
pub struct MelodyInfo {
    /// The melody ID (e.g., "JY-Tokyo")
    pub id: &'static str,
    /// The line code (e.g., "JY")
    pub line: &'static str,
    /// The line name in English
    pub line_name: &'static str,
    /// The station name in English
    pub station: &'static str,
    /// The station name in Japanese
    pub station_jp: &'static str,
    /// The melody name
    pub melody_name: &'static str,
    /// The audio file name (without base URL)
    pub filename: &'static str,
}

impl MelodyInfo {
    /// Get the full URL for downloading this melody
    pub fn url(&self) -> String {
        format!("{}{}", YAMANOTES_BASE_URL, self.filename)
    }
}

/// Base URL for yamanot.es audio files
pub const YAMANOTES_BASE_URL: &str = "https://yamanot.es/audio/";

/// All predefined melodies from JR East lines
pub const MELODIES: &[MelodyInfo] = &[
    // ============================================
    // JY: Yamanote Line (山手線)
    // ============================================
    MelodyInfo {
        id: "JY-Tokyo",
        line: "JY",
        line_name: "Yamanote",
        station: "Tokyo",
        station_jp: "東京",
        melody_name: "SH-3",
        filename: "sh3.mp3",
    },
    MelodyInfo {
        id: "JY-Kanda",
        line: "JY",
        line_name: "Yamanote",
        station: "Kanda",
        station_jp: "神田",
        melody_name: "Seseragi",
        filename: "seseragi.mp3",
    },
    MelodyInfo {
        id: "JY-Akihabara",
        line: "JY",
        line_name: "Yamanote",
        station: "Akihabara",
        station_jp: "秋葉原",
        melody_name: "Ogawa V1",
        filename: "ogawav1.mp3",
    },
    MelodyInfo {
        id: "JY-Okachimachi",
        line: "JY",
        line_name: "Yamanote",
        station: "Okachimachi",
        station_jp: "御徒町",
        melody_name: "Haru Tremolo",
        filename: "harutrem.mp3",
    },
    MelodyInfo {
        id: "JY-Ueno",
        line: "JY",
        line_name: "Yamanote",
        station: "Ueno",
        station_jp: "上野",
        melody_name: "Bell B",
        filename: "bellb.mp3",
    },
    MelodyInfo {
        id: "JY-Uguisudani",
        line: "JY",
        line_name: "Yamanote",
        station: "Uguisudani",
        station_jp: "鶯谷",
        melody_name: "Haru Tremolo",
        filename: "harutrem.mp3",
    },
    MelodyInfo {
        id: "JY-Nippori",
        line: "JY",
        line_name: "Yamanote",
        station: "Nippori",
        station_jp: "日暮里",
        melody_name: "Haru Tremolo",
        filename: "harutrem.mp3",
    },
    MelodyInfo {
        id: "JY-NishiNippori",
        line: "JY",
        line_name: "Yamanote",
        station: "Nishi-Nippori",
        station_jp: "西日暮里",
        melody_name: "Haru Tremolo",
        filename: "harutrem.mp3",
    },
    MelodyInfo {
        id: "JY-Tabata",
        line: "JY",
        line_name: "Yamanote",
        station: "Tabata",
        station_jp: "田端",
        melody_name: "Haru Tremolo",
        filename: "harutrem.mp3",
    },
    MelodyInfo {
        id: "JY-Komagome",
        line: "JY",
        line_name: "Yamanote",
        station: "Komagome",
        station_jp: "駒込",
        melody_name: "Sakura B",
        filename: "sakurab.mp3",
    },
    MelodyInfo {
        id: "JY-Sugamo",
        line: "JY",
        line_name: "Yamanote",
        station: "Sugamo",
        station_jp: "巣鴨",
        melody_name: "Haru",
        filename: "haru.mp3",
    },
    MelodyInfo {
        id: "JY-Otsuka",
        line: "JY",
        line_name: "Yamanote",
        station: "Otsuka",
        station_jp: "大塚",
        melody_name: "Haru",
        filename: "haru.mp3",
    },
    MelodyInfo {
        id: "JY-Ikebukuro",
        line: "JY",
        line_name: "Yamanote",
        station: "Ikebukuro",
        station_jp: "池袋",
        melody_name: "Melody",
        filename: "melody.mp3",
    },
    MelodyInfo {
        id: "JY-Mejiro",
        line: "JY",
        line_name: "Yamanote",
        station: "Mejiro",
        station_jp: "目白",
        melody_name: "Haru",
        filename: "haru.mp3",
    },
    MelodyInfo {
        id: "JY-Takadanobaba",
        line: "JY",
        line_name: "Yamanote",
        station: "Takadanobaba",
        station_jp: "高田馬場",
        melody_name: "Astro Boy",
        filename: "astrob.mp3",
    },
    MelodyInfo {
        id: "JY-ShinOkubo",
        line: "JY",
        line_name: "Yamanote",
        station: "Shin-Okubo",
        station_jp: "新大久保",
        melody_name: "Bell B",
        filename: "bellb.mp3",
    },
    MelodyInfo {
        id: "JY-Shinjuku",
        line: "JY",
        line_name: "Yamanote",
        station: "Shinjuku",
        station_jp: "新宿",
        melody_name: "Aratana",
        filename: "aratana.mp3",
    },
    MelodyInfo {
        id: "JY-Yoyogi",
        line: "JY",
        line_name: "Yamanote",
        station: "Yoyogi",
        station_jp: "代々木",
        melody_name: "Haru",
        filename: "haru.mp3",
    },
    MelodyInfo {
        id: "JY-Harajuku",
        line: "JY",
        line_name: "Yamanote",
        station: "Harajuku",
        station_jp: "原宿",
        melody_name: "Harajuku A",
        filename: "harajukua.mp3",
    },
    MelodyInfo {
        id: "JY-Shibuya",
        line: "JY",
        line_name: "Yamanote",
        station: "Shibuya",
        station_jp: "渋谷",
        melody_name: "Hana no Horokobi",
        filename: "hananohorokobi.mp3",
    },
    MelodyInfo {
        id: "JY-Ebisu",
        line: "JY",
        line_name: "Yamanote",
        station: "Ebisu",
        station_jp: "恵比寿",
        melody_name: "Third Man",
        filename: "thirdman.mp3",
    },
    MelodyInfo {
        id: "JY-Meguro",
        line: "JY",
        line_name: "Yamanote",
        station: "Meguro",
        station_jp: "目黒",
        melody_name: "Water Crown",
        filename: "watercrown.mp3",
    },
    MelodyInfo {
        id: "JY-Gotanda",
        line: "JY",
        line_name: "Yamanote",
        station: "Gotanda",
        station_jp: "五反田",
        melody_name: "SH-23",
        filename: "sh23.mp3",
    },
    MelodyInfo {
        id: "JY-Osaki",
        line: "JY",
        line_name: "Yamanote",
        station: "Osaki",
        station_jp: "大崎",
        melody_name: "Umi no Eki",
        filename: "uminoeki.mp3",
    },
    MelodyInfo {
        id: "JY-Shinagawa",
        line: "JY",
        line_name: "Yamanote",
        station: "Shinagawa",
        station_jp: "品川",
        melody_name: "Seseragi",
        filename: "seseragi.mp3",
    },
    MelodyInfo {
        id: "JY-TakanawaGateway",
        line: "JY",
        line_name: "Yamanote",
        station: "Takanawa Gateway",
        station_jp: "高輪ゲートウェイ",
        melody_name: "Sweet Call",
        filename: "sweetcall.mp3",
    },
    MelodyInfo {
        id: "JY-Tamachi",
        line: "JY",
        line_name: "Yamanote",
        station: "Tamachi",
        station_jp: "田町",
        melody_name: "Seseragi",
        filename: "seseragi.mp3",
    },
    MelodyInfo {
        id: "JY-Hamamatsucho",
        line: "JY",
        line_name: "Yamanote",
        station: "Hamamatsucho",
        station_jp: "浜松町",
        melody_name: "Seseragi",
        filename: "seseragi.mp3",
    },
    MelodyInfo {
        id: "JY-Shimbashi",
        line: "JY",
        line_name: "Yamanote",
        station: "Shimbashi",
        station_jp: "新橋",
        melody_name: "Gota del Vient",
        filename: "gotadelvient.mp3",
    },
    MelodyInfo {
        id: "JY-Yurakucho",
        line: "JY",
        line_name: "Yamanote",
        station: "Yurakucho",
        station_jp: "有楽町",
        melody_name: "SH-21",
        filename: "sh21.mp3",
    },
    // ============================================
    // JK: Keihin-Tohoku Line (京浜東北線)
    // ============================================
    MelodyInfo {
        id: "JK-Shinagawa",
        line: "JK",
        line_name: "Keihin-Tohoku",
        station: "Shinagawa",
        station_jp: "品川",
        melody_name: "Chime",
        filename: "chime.mp3",
    },
    MelodyInfo {
        id: "JK-TakanawaGateway",
        line: "JK",
        line_name: "Keihin-Tohoku",
        station: "Takanawa Gateway",
        station_jp: "高輪ゲートウェイ",
        melody_name: "Flower Shop",
        filename: "flowershop.mp3",
    },
    MelodyInfo {
        id: "JK-Tamachi",
        line: "JK",
        line_name: "Keihin-Tohoku",
        station: "Tamachi",
        station_jp: "田町",
        melody_name: "Spring Box",
        filename: "springbox.mp3",
    },
    MelodyInfo {
        id: "JK-Hamamatsucho",
        line: "JK",
        line_name: "Keihin-Tohoku",
        station: "Hamamatsucho",
        station_jp: "浜松町",
        melody_name: "Spring Box",
        filename: "springbox.mp3",
    },
    MelodyInfo {
        id: "JK-Shimbashi",
        line: "JK",
        line_name: "Keihin-Tohoku",
        station: "Shimbashi",
        station_jp: "新橋",
        melody_name: "SH-1",
        filename: "sh1.mp3",
    },
    MelodyInfo {
        id: "JK-Yurakucho",
        line: "JK",
        line_name: "Keihin-Tohoku",
        station: "Yurakucho",
        station_jp: "有楽町",
        melody_name: "SH-5",
        filename: "sh5.mp3",
    },
    MelodyInfo {
        id: "JK-Tokyo",
        line: "JK",
        line_name: "Keihin-Tohoku",
        station: "Tokyo",
        station_jp: "東京",
        melody_name: "SH-5",
        filename: "sh5.mp3",
    },
    MelodyInfo {
        id: "JK-Kanda",
        line: "JK",
        line_name: "Keihin-Tohoku",
        station: "Kanda",
        station_jp: "神田",
        melody_name: "Haru New",
        filename: "harunew.mp3",
    },
    MelodyInfo {
        id: "JK-Akihabara",
        line: "JK",
        line_name: "Keihin-Tohoku",
        station: "Akihabara",
        station_jp: "秋葉原",
        melody_name: "Beyond the Line",
        filename: "beyondtheline.mp3",
    },
    MelodyInfo {
        id: "JK-Okachimachi",
        line: "JK",
        line_name: "Keihin-Tohoku",
        station: "Okachimachi",
        station_jp: "御徒町",
        melody_name: "Haru New",
        filename: "harunew.mp3",
    },
    MelodyInfo {
        id: "JK-Ueno",
        line: "JK",
        line_name: "Keihin-Tohoku",
        station: "Ueno",
        station_jp: "上野",
        melody_name: "Bell A",
        filename: "bella.mp3",
    },
    MelodyInfo {
        id: "JK-Uguisudani",
        line: "JK",
        line_name: "Keihin-Tohoku",
        station: "Uguisudani",
        station_jp: "鶯谷",
        melody_name: "Haru New",
        filename: "harunew.mp3",
    },
    MelodyInfo {
        id: "JK-Nippori",
        line: "JK",
        line_name: "Keihin-Tohoku",
        station: "Nippori",
        station_jp: "日暮里",
        melody_name: "Haru New",
        filename: "harunew.mp3",
    },
    MelodyInfo {
        id: "JK-NishiNippori",
        line: "JK",
        line_name: "Keihin-Tohoku",
        station: "Nishi-Nippori",
        station_jp: "西日暮里",
        melody_name: "Haru New",
        filename: "harunew.mp3",
    },
    MelodyInfo {
        id: "JK-Tabata",
        line: "JK",
        line_name: "Keihin-Tohoku",
        station: "Tabata",
        station_jp: "田端",
        melody_name: "Haru New",
        filename: "harunew.mp3",
    },
    // ============================================
    // JB: Sobu Line (総武線)
    // ============================================
    MelodyInfo {
        id: "JB-Ichigaya",
        line: "JB",
        line_name: "Sobu",
        station: "Ichigaya",
        station_jp: "市ケ谷",
        melody_name: "Haru New",
        filename: "harunew.mp3",
    },
    MelodyInfo {
        id: "JB-Iidabashi",
        line: "JB",
        line_name: "Sobu",
        station: "Iidabashi",
        station_jp: "飯田橋",
        melody_name: "SF-3",
        filename: "sf3.mp3",
    },
    MelodyInfo {
        id: "JB-Suidobashi",
        line: "JB",
        line_name: "Sobu",
        station: "Suidobashi",
        station_jp: "水道橋",
        melody_name: "Fighting Spirit A",
        filename: "fightingspirita.mp3",
    },
    MelodyInfo {
        id: "JB-Ochanomizu",
        line: "JB",
        line_name: "Sobu",
        station: "Ochanomizu",
        station_jp: "御茶ノ水",
        melody_name: "SH-6",
        filename: "sh6.mp3",
    },
    MelodyInfo {
        id: "JB-Akihabara",
        line: "JB",
        line_name: "Sobu",
        station: "Akihabara",
        station_jp: "秋葉原",
        melody_name: "SF-3",
        filename: "sf3.mp3",
    },
    // ============================================
    // JA: Saikyo Line (埼京線)
    // ============================================
    MelodyInfo {
        id: "JA-Ikebukuro",
        line: "JA",
        line_name: "Saikyo",
        station: "Ikebukuro",
        station_jp: "池袋",
        melody_name: "Mellow Time",
        filename: "mellowtime.mp3",
    },
    MelodyInfo {
        id: "JA-Shinjuku",
        line: "JA",
        line_name: "Saikyo",
        station: "Shinjuku",
        station_jp: "新宿",
        melody_name: "Mellow Time",
        filename: "mellowtime.mp3",
    },
    MelodyInfo {
        id: "JA-Shibuya",
        line: "JA",
        line_name: "Saikyo",
        station: "Shibuya",
        station_jp: "渋谷",
        melody_name: "SH-1",
        filename: "sh1.mp3",
    },
    MelodyInfo {
        id: "JA-Ebisu",
        line: "JA",
        line_name: "Saikyo",
        station: "Ebisu",
        station_jp: "恵比寿",
        melody_name: "Third Man",
        filename: "thirdman.mp3",
    },
    MelodyInfo {
        id: "JA-Osaki",
        line: "JA",
        line_name: "Saikyo",
        station: "Osaki",
        station_jp: "大崎",
        melody_name: "Twinkling Skyline",
        filename: "twinklingskyline.mp3",
    },
    // ============================================
    // JU: Ueno-Tokyo Line (上野東京ライン)
    // ============================================
    MelodyInfo {
        id: "JU-Shinagawa",
        line: "JU",
        line_name: "Ueno-Tokyo",
        station: "Shinagawa",
        station_jp: "品川",
        melody_name: "Railroad Song B",
        filename: "railroadsongb.mp3",
    },
    MelodyInfo {
        id: "JU-Shimbashi",
        line: "JU",
        line_name: "Ueno-Tokyo",
        station: "Shimbashi",
        station_jp: "新橋",
        melody_name: "Sunlight",
        filename: "sunlight.mp3",
    },
    MelodyInfo {
        id: "JU-Tokyo",
        line: "JU",
        line_name: "Ueno-Tokyo",
        station: "Tokyo",
        station_jp: "東京",
        melody_name: "Bell A",
        filename: "bella.mp3",
    },
    MelodyInfo {
        id: "JU-Ueno",
        line: "JU",
        line_name: "Ueno-Tokyo",
        station: "Ueno",
        station_jp: "上野",
        melody_name: "Beyond the Line",
        filename: "beyondtheline.mp3",
    },
    // ============================================
    // NEX: Narita Express (成田エクスプレス)
    // ============================================
    MelodyInfo {
        id: "NEX-Ikebukuro",
        line: "NEX",
        line_name: "Narita Express",
        station: "Ikebukuro",
        station_jp: "池袋",
        melody_name: "Haru",
        filename: "haru.mp3",
    },
    MelodyInfo {
        id: "NEX-Shinjuku",
        line: "NEX",
        line_name: "Narita Express",
        station: "Shinjuku",
        station_jp: "新宿",
        melody_name: "Beautiful Hill",
        filename: "beautifulhill.mp3",
    },
    MelodyInfo {
        id: "NEX-Shibuya",
        line: "NEX",
        line_name: "Narita Express",
        station: "Shibuya",
        station_jp: "渋谷",
        melody_name: "SH-1",
        filename: "sh1.mp3",
    },
    MelodyInfo {
        id: "NEX-Shinagawa",
        line: "NEX",
        line_name: "Narita Express",
        station: "Shinagawa",
        station_jp: "品川",
        melody_name: "Seseragi",
        filename: "seseragi.mp3",
    },
];

/// Registry for looking up melodies by ID
pub struct MelodyRegistry {
    melodies: HashMap<String, &'static MelodyInfo>,
}

impl MelodyRegistry {
    /// Create a new melody registry
    pub fn new() -> Self {
        let mut melodies = HashMap::new();
        for melody in MELODIES {
            // Add with exact ID
            melodies.insert(melody.id.to_string(), melody);
            // Add with lowercase ID for case-insensitive lookup
            melodies.insert(melody.id.to_lowercase(), melody);
        }
        Self { melodies }
    }

    /// Look up a melody by ID (case-insensitive)
    pub fn get(&self, id: &str) -> Option<&'static MelodyInfo> {
        self.melodies
            .get(id)
            .or_else(|| self.melodies.get(&id.to_lowercase()))
            .copied()
    }

    /// Get all melodies
    #[allow(dead_code)]
    pub fn all(&self) -> &'static [MelodyInfo] {
        MELODIES
    }
}

impl Default for MelodyRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_lookup() {
        let registry = MelodyRegistry::new();

        // Yamanote Line
        let melody = registry.get("JY-Tokyo").unwrap();
        assert_eq!(melody.station, "Tokyo");
        assert_eq!(melody.line, "JY");

        // Keihin-Tohoku Line
        let melody = registry.get("JK-Akihabara").unwrap();
        assert_eq!(melody.melody_name, "Beyond the Line");

        // Sobu Line
        let melody = registry.get("JB-Ochanomizu").unwrap();
        assert_eq!(melody.melody_name, "SH-6");

        // Saikyo Line
        let melody = registry.get("JA-Osaki").unwrap();
        assert_eq!(melody.melody_name, "Twinkling Skyline");

        // Ueno-Tokyo Line
        let melody = registry.get("JU-Tokyo").unwrap();
        assert_eq!(melody.melody_name, "Bell A");

        // Narita Express
        let melody = registry.get("NEX-Shinjuku").unwrap();
        assert_eq!(melody.melody_name, "Beautiful Hill");

        // Case-insensitive
        let melody = registry.get("jy-tokyo").unwrap();
        assert_eq!(melody.station, "Tokyo");

        // Not found
        assert!(registry.get("XX-Unknown").is_none());
    }

    #[test]
    fn test_melody_url() {
        let melody = &MELODIES[0]; // JY-Tokyo
        assert_eq!(melody.url(), "https://yamanot.es/audio/sh3.mp3");
    }
}
