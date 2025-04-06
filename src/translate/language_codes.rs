use strum::{IntoEnumIterator};
use strum_macros::EnumIter;
#[derive(EnumIter, Debug)]
pub enum LanguageCode {
    Abkhaz,
    Acehnese,
    Acholi,
    Afrikaans,
    Albanian,
    Alur,
    Amharic,
    Arabic,
    Armenian,
    Assamese,
    Awadhi,
    Aymara,
    Azerbaijani,
    Balinese,
    Bambara,
    Bashkir,
    Basque,
    BatakKaro,
    BatakSimalungun,
    BatakToba,
    Belarusian,
    Bemba,
    Bengali,
    Betawi,
    Bhojpuri,
    Bikol,
    Bosnian,
    Breton,
    Bulgarian,
    Buryat,
    Cantonese,
    Catalan,
    Cebuano,
    ChichewaNyanja,
    ChineseSimplified, // zh (BCP-47)
    ChineseTraditional,
    Chuvash,
    Corsican,
    CrimeanTatar,
    Croatian,
    Czech,
    Danish,
    Dinka,
    Divehi,
    Dogri,
    Dombe,
    Dutch,
    Dzongkha,
    English,
    Esperanto,
    Estonian,
    Ewe,
    Fijian,
    FilipinoTagalog,  // or tl
    Finnish,
    French,
    // French,
    FrenchCanadian,
    Frisian,
    Fulfulde,
    Ga,
    Galician,
    GandaLuganda,
    Georgian,
    German,
    Greek,
    Guarani,
    Gujarati,
    HaitianCreole,
    HakhaChin,
    Hausa,
    Hawaiian,
    Hebrew, // or he
    Hiligaynon,
    Hindi,
    Hmong,
    Hungarian,
    Hunsrik,
    Icelandic,
    Igbo,
    Iloko,
    Indonesian,
    Irish,
    Italian,
    Japanese,
    Javanese, // or jv
    Kannada,
    Kapampangan,
    Kazakh,
    Khmer,
    Kiga,
    Kinyarwanda,
    Kituba,
    Konkani,
    Korean,
    Krio,
    KurdishKurmanji,
    KurdishSorani,
    Kyrgyz,
    Lao,
    Latgalian,
    Latin,
    Latvian,
    Ligurian,
    Limburgan,
    Lingala,
    Lithuanian,
    Lombard,
    Luo,
    Luxembourgish,
    Macedonian,
    Maithili,
    Makassar,
    Malagasy,
    Malay,
    MalayJawi,
    Malayalam,
    Maltese,
    Maori,
    Marathi,
    MeadowMari,
    MeiteilonManipuri,
    Minang,
    Mizo,
    Mongolian,
    MyanmarBurmese,
    NdebeleSouth,
    NepalbhasaNewari,
    Nepali,
    NorthernSotho,
    Norwegian,
    Nuer,
    Occitan,
    OdiaOriya,
    Oromo,
    Pangasinan,
    Papiamento,
    Pashto,
    Persian,
    Polish,
    Portuguese,
    PortuguesePortugal,
    PortugueseBrazil,
    Punjabi,
    PunjabiShahmukhi,
    Quechua,
    Romani,
    Romanian,
    Rundi,
    Russian,
    Samoan,
    Sango,
    Sanskrit,
    ScotsGaelic,
    Serbian,
    Sesotho,
    SeychelloisCreole,
    Shan,
    Shona,
    Sicilian,
    Silesian,
    Sindhi,
    SinhalaSinhalese,
    Slovak,
    Slovenian,
    Somali,
    Spanish,
    Sundanese,
    Swahili,
    Swati,
    Swedish,
    Tajik,
    Tamil,
    Tatar,
    Telugu,
    Tetum,
    Thai,
    Tigrinya,
    Tsonga,
    Tswana,
    Turkish,
    Turkmen,
    TwiAkan,
    Ukrainian,
    Urdu,
    Uyghur,
    Uzbek,
    Vietnamese,
    Welsh,
    Xhosa,
    Yiddish,
    Yoruba,
    YucatecMaya,
    Zulu
}

impl LanguageCode {
    pub fn values() -> Vec<LanguageCode> {
        LanguageCode::iter().collect()
    }
    fn get_code(&self) -> &'static str {
        match self {
            Self::Abkhaz => "ab",
            Self::Acehnese => "ace",
            Self::Acholi => "ach",
            Self::Afrikaans => "af",
            Self::Albanian => "sq",
            Self::Alur => "alz",
            Self::Amharic => "am",
            Self::Arabic => "ar",
            Self::Armenian => "hy",
            Self::Assamese => "as",
            Self::Awadhi => "awa",
            Self::Aymara => "ay",
            Self::Azerbaijani => "az",
            Self::Balinese => "ban",
            Self::Bambara => "bm",
            Self::Bashkir => "ba",
            Self::Basque => "eu",
            Self::BatakKaro => "btx",
            Self::BatakSimalungun => "bts",
            Self::BatakToba => "bbc",
            Self::Belarusian => "be",
            Self::Bemba => "bem",
            Self::Bengali => "bn",
            Self::Betawi => "bew",
            Self::Bhojpuri => "bho",
            Self::Bikol => "bik",
            Self::Bosnian => "bs",
            Self::Breton => "br",
            Self::Bulgarian => "bg",
            Self::Buryat => "bua",
            Self::Cantonese => "yue",
            Self::Catalan => "ca",
            Self::Cebuano => "ceb",
            Self::ChichewaNyanja => "ny",
            Self::ChineseSimplified => "zh-CN", // zh (BCP-47)
            Self::ChineseTraditional => "zh-TW",
            Self::Chuvash => "cv",
            Self::Corsican => "co",
            Self::CrimeanTatar => "crh",
            Self::Croatian => "hr",
            Self::Czech => "cs",
            Self::Danish => "da",
            Self::Dinka => "din",
            Self::Divehi => "dv",
            Self::Dogri => "doi",
            Self::Dombe => "dov",
            Self::Dutch => "nl",
            Self::Dzongkha => "dz",
            Self::English => "en",
            Self::Esperanto => "eo",
            Self::Estonian => "et",
            Self::Ewe => "ee",
            Self::Fijian => "fj",
            Self::FilipinoTagalog => "fil",  // or tl
            Self::Finnish => "fi",
            Self::French => "fr",
            // Self::French => "fr-FR",
            Self::FrenchCanadian => "fr-CA",
            Self::Frisian => "fy",
            Self::Fulfulde => "ff",
            Self::Ga => "gaa",
            Self::Galician => "gl",
            Self::GandaLuganda => "lg",
            Self::Georgian => "ka",
            Self::German => "de",
            Self::Greek => "el",
            Self::Guarani => "gn",
            Self::Gujarati => "gu",
            Self::HaitianCreole => "ht",
            Self::HakhaChin => "cnh",
            Self::Hausa => "ha",
            Self::Hawaiian => "haw",
            Self::Hebrew => "iw", // or he
            Self::Hiligaynon => "hil",
            Self::Hindi => "hi",
            Self::Hmong => "hmn",
            Self::Hungarian => "hu",
            Self::Hunsrik => "hrx",
            Self::Icelandic => "is",
            Self::Igbo => "ig",
            Self::Iloko => "ilo",
            Self::Indonesian => "id",
            Self::Irish => "ga",
            Self::Italian => "it",
            Self::Japanese => "ja",
            Self::Javanese => "jw", // or jv
            Self::Kannada => "kn",
            Self::Kapampangan => "pam",
            Self::Kazakh => "kk",
            Self::Khmer => "km",
            Self::Kiga => "cgg",
            Self::Kinyarwanda => "rw",
            Self::Kituba => "ktu",
            Self::Konkani => "gom",
            Self::Korean => "ko",
            Self::Krio => "kri",
            Self::KurdishKurmanji => "ku",
            Self::KurdishSorani => "ckb",
            Self::Kyrgyz => "ky",
            Self::Lao => "lo",
            Self::Latgalian => "ltg",
            Self::Latin => "la",
            Self::Latvian => "lv",
            Self::Ligurian => "lij",
            Self::Limburgan => "li",
            Self::Lingala => "ln",
            Self::Lithuanian => "lt",
            Self::Lombard => "lmo",
            Self::Luo => "luo",
            Self::Luxembourgish => "lb",
            Self::Macedonian => "mk",
            Self::Maithili => "mai",
            Self::Makassar => "mak",
            Self::Malagasy => "mg",
            Self::Malay => "ms",
            Self::MalayJawi => "ms-Arab",
            Self::Malayalam => "ml",
            Self::Maltese => "mt",
            Self::Maori => "mi",
            Self::Marathi => "mr",
            Self::MeadowMari => "chm",
            Self::MeiteilonManipuri => "mni-Mtei",
            Self::Minang => "min",
            Self::Mizo => "lus",
            Self::Mongolian => "mn",
            Self::MyanmarBurmese => "my",
            Self::NdebeleSouth => "nr",
            Self::NepalbhasaNewari => "new",
            Self::Nepali => "ne",
            Self::NorthernSotho => "nso",
            Self::Norwegian => "no",
            Self::Nuer => "nus",
            Self::Occitan => "oc",
            Self::OdiaOriya => "or",
            Self::Oromo => "om",
            Self::Pangasinan => "pag",
            Self::Papiamento => "pap",
            Self::Pashto => "ps",
            Self::Persian => "fa",
            Self::Polish => "pl",
            Self::Portuguese => "pt",
            Self::PortuguesePortugal => "pt-PT",
            Self::PortugueseBrazil => "pt-BR",
            Self::Punjabi => "pa",
            Self::PunjabiShahmukhi => "pa-Arab",
            Self::Quechua => "qu",
            Self::Romani => "rom",
            Self::Romanian => "ro",
            Self::Rundi => "rn",
            Self::Russian => "ru",
            Self::Samoan => "sm",
            Self::Sango => "sg",
            Self::Sanskrit => "sa",
            Self::ScotsGaelic => "gd",
            Self::Serbian => "sr",
            Self::Sesotho => "st",
            Self::SeychelloisCreole => "crs",
            Self::Shan => "shn",
            Self::Shona => "sn",
            Self::Sicilian => "scn",
            Self::Silesian => "szl",
            Self::Sindhi => "sd",
            Self::SinhalaSinhalese => "si",
            Self::Slovak => "sk",
            Self::Slovenian => "sl",
            Self::Somali => "so",
            Self::Spanish => "es",
            Self::Sundanese => "su",
            Self::Swahili => "sw",
            Self::Swati => "ss",
            Self::Swedish => "sv",
            Self::Tajik => "tg",
            Self::Tamil => "ta",
            Self::Tatar => "tt",
            Self::Telugu => "te",
            Self::Tetum => "tet",
            Self::Thai => "th",
            Self::Tigrinya => "ti",
            Self::Tsonga => "ts",
            Self::Tswana => "tn",
            Self::Turkish => "tr",
            Self::Turkmen => "tk",
            Self::TwiAkan => "ak",
            Self::Ukrainian => "uk",
            Self::Urdu => "ur",
            Self::Uyghur => "ug",
            Self::Uzbek => "uz",
            Self::Vietnamese => "vi",
            Self::Welsh => "cy",
            Self::Xhosa => "xh",
            Self::Yiddish => "yi",
            Self::Yoruba => "yo",
            Self::YucatecMaya => "yua",
            Self::Zulu => "zu"
        }
    }
}