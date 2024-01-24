/// ISO 4217 currency codes <https://en.wikipedia.org/wiki/ISO_4217>
#[repr(u32)]
#[rustfmt::skip]
#[allow(clippy::zero_prefixed_literal)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum CurrencyCode {
    /// United Arab Emirates dirham United Arab Emirates
    AED = 784,
    /// Afghan afghani Afghanistan
    AFN = 971,
    /// Albanian lek Albania
    ALL = 008,
    /// Armenian dram Armenia
    AMD = 051,
    /// Netherlands Antillean guilder Cura
    ANG = 532,
    /// Angolan kwanza Angola
    AOA = 973,
    /// Argentine peso Argentina
    ARS = 032,
    /// Australian dollar Australia
    AUD = 036,
    /// Aruban florin Aruba
    AWG = 533,
    /// Azerbaijani manat Azerbaijan
    AZN = 944,
    /// Bosnia and Herzegovina convertible mark Bosnia and Herzegovina
    BAM = 977,
    /// Barbados dollar Barbados
    BBD = 052,
    /// Bangladeshi taka Bangladesh
    BDT = 050,
    /// Bulgarian lev Bulgaria
    BGN = 975,
    /// Bahraini dinar Bahrain
    BHD = 048,
    /// Burundian franc Burundi
    BIF = 108,
    /// Bermudian dollar Bermuda
    BMD = 060,
    /// Brunei dollar Brunei
    BND = 096,
    /// Boliviano Bolivia
    BOB = 068,
    /// Bolivian Mvdol (funds code)
    BOV = 984,
    /// Brazilian real Brazil
    BRL = 986,
    /// Bahamian dollar Bahamas
    BSD = 044,
    /// Bhutanese ngultrum Bhutan
    BTN = 064,
    /// Botswana pula Botswana
    BWP = 072,
    /// Belarusian ruble Belarus
    BYN = 933,
    /// Belize dollar Belize
    BZD = 084,
    /// Canadian dollar Canada
    CAD = 124,
    /// Congolese franc Democratic Republic of the Congo
    CDF = 976,
    /// WIR euro (complementary currency)
    CHE = 947,
    /// Swiss franc Switzerland
    CHF = 756,
    /// WIR franc (complementary currency)
    CHW = 948,
    /// Unidad de Fomento (funds code)
    CLF = 990,
    /// Chilean peso Chile
    CLP = 152,
    /// Colombian peso Colombia
    COP = 170,
    ///  Unidad de Valor Real (UVR) (funds code)
    COU = 970,
    /// Costa Rican colon Costa Rica
    CRC = 188,
    /// Cuban convertible peso Cuba
    CUC = 931,
    /// Cuban peso Cuba
    CUP = 192,
    /// Cape Verdean escudo Cabo Verde
    CVE = 132,
    /// Czech koruna Czechia
    CZK = 203,
    /// Djiboutian franc Djibouti
    DJF = 262,
    /// Danish krone Denmark
    DKK = 208,
    /// Dominican peso Dominican Republic
    DOP = 214,
    /// Algerian dinar Algeria
    DZD = 012,
    /// Egyptian pound Egypt
    EGP = 818,
    /// Eritrean nakfa Eritrea
    ERN = 232,
    /// Ethiopian birr Ethiopia
    ETB = 230,
    /// Euro
    EUR = 978,
    /// Fiji dollar Fiji
    FJD = 242,
    /// Falkland Islands pound Falkland Islands (pegged to GBP)
    FKP = 238,
    /// Pound sterling United Kingdom
    GBP = 826,
    /// Georgian lari Georgia
    GEL = 981,
    /// Ghanaian cedi Ghana
    GHS = 936,
    /// Gibraltar pound Gibraltar (pegged to GBP)
    GIP = 292,
    /// Gambian dalasi Gambia
    GMD = 270,
    /// Guinean franc Guinea
    GNF = 324,
    /// Guatemalan quetzal Guatemala
    GTQ = 320,
    /// Guyanese dollar Guyana
    GYD = 328,
    /// Hong Kong dollar Hong Kong
    HKD = 344,
    /// Honduran lempira Honduras
    HNL = 340,
    /// Haitian gourde Haiti
    HTG = 332,
    /// Hungarian forint Hungary
    HUF = 348,
    /// Indonesian rupiah Indonesia
    IDR = 360,
    /// Israeli new shekel Israel
    ILS = 376,
    /// Indian rupee India
    INR = 356,
    /// Iraqi dinar Iraq
    IQD = 368,
    /// Iranian rial Iran
    IRR = 364,
    /// Icelandic kr
    ISK = 352,
    /// Jamaican dollar Jamaica
    JMD = 388,
    /// Jordanian dinar Jordan
    JOD = 400,
    /// Japanese yen Japan
    JPY = 392,
    /// Kenyan shilling Kenya
    KES = 404,
    /// Kyrgyzstani som Kyrgyzstan
    KGS = 417,
    /// Cambodian riel Cambodia
    KHR = 116,
    /// Comoro franc Comoros
    KMF = 174,
    /// North Korean won North Korea
    KPW = 408,
    /// South Korean won
    KRW = 410,
    /// Kuwaiti dinar Kuwait
    KWD = 414,
    /// Cayman Islands dollar Cayman Islands
    KYD = 136,
    /// Kazakhstani tenge Kazakhstan
    KZT = 398,
    /// Lao kip Laos
    LAK = 418,
    /// Lebanese pound Lebanon
    LBP = 422,
    /// Sri Lankan rupee Sri Lanka
    LKR = 144,
    /// Liberian dollar Liberia
    LRD = 430,
    /// Lesotho loti Lesotho
    LSL = 426,
    /// Libyan dinar Libya
    LYD = 434,
    /// Moroccan dirham Morocco
    MAD = 504,
    /// Moldovan leu Moldova
    MDL = 498,
    /// Malagasy ariary
    MGA = 969,
    /// Macedonian denar North Macedonia
    MKD = 807,
    /// Myanmar kyat Myanmar
    MMK = 104,
    /// Mongolian tögrög
    MNT = 496,
    /// Macanese pataca Macau
    MOP = 446,
    /// Mauritanian ouguiya
    MRU = 929,
    /// Mauritian rupee Mauritius
    MUR = 480,
    /// Maldivian rufiyaa Maldives
    MVR = 462,
    /// Malawian kwacha Malawi
    MWK = 454,
    /// Mexican peso Mexico
    MXN = 484,
    /// Mexican Unidad de Inversion (UDI) (funds code)
    MXV = 979,
    /// Malaysian ringgit Malaysia
    MYR = 458,
    /// Mozambican metical Mozambique
    MZN = 943,
    /// Namibian dollar Namibia (pegged to ZAR)
    NAD = 516,
    /// Nigerian naira Nigeria
    NGN = 566,
    /// Nicaraguan c
    NIO = 558,
    /// Norwegian krone Norway
    NOK = 578,
    /// Nepalese rupee Nepal
    NPR = 524,
    /// New Zealand dollar New Zealand
    NZD = 554,
    /// Omani rial Oman
    OMR = 512,
    /// Panamanian balboa Panama
    PAB = 590,
    /// Peruvian sol Peru
    PEN = 604,
    /// Papua New Guinean kina Papua New Guinea
    PGK = 598,
    /// Philippine peso
    PHP = 608,
    /// Pakistani rupee Pakistan
    PKR = 586,
    /// Polish z
    PLN = 985,
    /// Paraguayan guaran
    PYG = 600,
    /// Qatari riyal Qatar
    QAR = 634,
    /// Romanian leu Romania
    RON = 946,
    /// Serbian dinar Serbia
    RSD = 941,
    /// Renminbi
    CNY = 156,
    /// Russian ruble Russia
    RUB = 643,
    /// Rwandan franc Rwanda
    RWF = 646,
    /// Saudi riyal Saudi Arabia
    SAR = 682,
    /// Solomon Islands dollar Solomon Islands
    SBD = 090,
    /// Seychelles rupee Seychelles
    SCR = 690,
    /// Sudanese pound Sudan
    SDG = 938,
    /// Swedish krona (plural
    SEK = 752,
    /// Singapore dollar Singapore
    SGD = 702,
    /// Saint Helena pound Saint Helena (SH
    SHP = 654,
    /// Sierra Leonean leone (new leone)
    SLE = 925,
    /// Sierra Leonean leone (old leone)
    SLL = 694,
    /// Somali shilling Somalia
    SOS = 706,
    /// Surinamese dollar Suriname
    SRD = 968,
    /// South Sudanese pound South Sudan
    SSP = 728,
    /// São Tomé and Príncipe dobra
    STN = 930,
    /// Salvadoran col
    SVC = 222,
    /// Syrian pound Syria
    SYP = 760,
    /// Swazi lilangeni Eswatini
    SZL = 748,
    /// Thai baht Thailand
    THB = 764,
    /// Tajikistani somoni Tajikistan
    TJS = 972,
    /// Turkmenistan manat Turkmenistan
    TMT = 934,
    /// Tunisian dinar Tunisia
    TND = 788,
    /// Tongan pa
    TOP = 776,
    /// Turkish lira Turkey
    TRY = 949,
    /// Trinidad and Tobago dollar Trinidad and Tobago
    TTD = 780,
    /// New Taiwan dollar Taiwan
    TWD = 901,
    /// Tanzanian shilling Tanzania
    TZS = 834,
    /// Ukrainian hryvnia Ukraine
    UAH = 980,
    /// Ugandan shilling Uganda
    UGX = 800,
    /// United States dollar United States
    USD = 840,
    /// United States dollar (next day) (funds code)
    USN = 997,
    /// Uruguay Peso en Unidades Indexadas (URUIURUI) (funds code)
    UYI = 940,
    /// Uruguayan peso Uruguay
    UYU = 858,
    /// Unidad previsional
    UYW = 927,
    /// Uzbekistan sum Uzbekistan
    UZS = 860,
    /// Venezuelan digital bol
    VED = 926,
    /// Venezuelan sovereign bol
    VES = 928,
    /// Vietnamese
    VND = 704,
    /// Vanuatu vatu Vanuatu
    VUV = 548,
    /// Samoan tala Samoa
    WST = 882,
    /// CFA franc BEAC Cameroon (CM)
    XAF = 950,
    /// Silver (one troy ounce)
    XAG = 961,
    /// Gold (one troy ounce)
    XAU = 959,
    /// European Composite Unit (bond market unit)
    XBA = 955,
    /// European Monetary Unit (bond market unit)
    XBB = 956,
    /// European Unit of Account 9 (bond market unit)
    XBC = 957,
    /// European unit of account 17 (bond market unit)
    XBD = 958,
    /// East Caribbean dollar Anguilla (AI)
    XCD = 951,
    /// Special drawing rights
    XDR = 960,
    /// CFA franc BCEAO Benin (BJ)
    XOF = 952,
    /// Palladium (one troy ounce)
    XPD = 964,
    /// CFP franc (franc Pacifique)
    XPF = 953,
    /// Platinum (one troy ounce)
    XPT = 962,
    /// SUCRE
    XSU = 994,
    /// Code reserved for testing
    XTS = 963,
    /// ADB Unit of Account
    XUA = 965,
    /// No country code
    #[default]
    XXX = 999,
    /// Yemeni rial Yemen
    YER = 886,
    /// South African rand Eswatini
    ZAR = 710,
    /// Zambian kwacha Zambia
    ZMW = 967,
    /// Zimbabwean dollar (fifth)
    ZWL = 932,
}

impl CurrencyCode {
    /// Creates a new [CurrencyCode].
    pub const fn new() -> Self {
        Self::XXX
    }
}

impl From<CurrencyCode> for u32 {
    fn from(val: CurrencyCode) -> Self {
        val as u32
    }
}

impl From<&CurrencyCode> for u32 {
    fn from(val: &CurrencyCode) -> Self {
        (*val).into()
    }
}

impl From<CurrencyCode> for [i8; 4] {
    fn from(val: CurrencyCode) -> Self {
        let cur_str = <&str>::from(val);
        let cur_bytes = cur_str.as_bytes();

        [
            cur_bytes[0] as i8,
            cur_bytes[1] as i8,
            cur_bytes[2] as i8,
            b'\0' as i8,
        ]
    }
}

impl From<&CurrencyCode> for [i8; 4] {
    fn from(val: &CurrencyCode) -> Self {
        (*val).into()
    }
}

impl From<CurrencyCode> for &'static str {
    fn from(val: CurrencyCode) -> Self {
        match val {
            CurrencyCode::AED => "AED",
            CurrencyCode::AFN => "AFN",
            CurrencyCode::ALL => "ALL",
            CurrencyCode::AMD => "AMD",
            CurrencyCode::ANG => "ANG",
            CurrencyCode::AOA => "AOA",
            CurrencyCode::ARS => "ARS",
            CurrencyCode::AUD => "AUD",
            CurrencyCode::AWG => "AWG",
            CurrencyCode::AZN => "AZN",
            CurrencyCode::BAM => "BAM",
            CurrencyCode::BBD => "BBD",
            CurrencyCode::BDT => "BDT",
            CurrencyCode::BGN => "BGN",
            CurrencyCode::BHD => "BHD",
            CurrencyCode::BIF => "BIF",
            CurrencyCode::BMD => "BMD",
            CurrencyCode::BND => "BND",
            CurrencyCode::BOB => "BOB",
            CurrencyCode::BOV => "BOV",
            CurrencyCode::BRL => "BRL",
            CurrencyCode::BSD => "BSD",
            CurrencyCode::BTN => "BTN",
            CurrencyCode::BWP => "BWP",
            CurrencyCode::BYN => "BYN",
            CurrencyCode::BZD => "BZD",
            CurrencyCode::CAD => "CAD",
            CurrencyCode::CDF => "CDF",
            CurrencyCode::CHE => "CHE",
            CurrencyCode::CHF => "CHF",
            CurrencyCode::CHW => "CHW",
            CurrencyCode::CLF => "CLF",
            CurrencyCode::CLP => "CLP",
            CurrencyCode::COP => "COP",
            CurrencyCode::COU => "COU",
            CurrencyCode::CRC => "CRC",
            CurrencyCode::CUC => "CUC",
            CurrencyCode::CUP => "CUP",
            CurrencyCode::CVE => "CVE",
            CurrencyCode::CZK => "CZK",
            CurrencyCode::DJF => "DJF",
            CurrencyCode::DKK => "DKK",
            CurrencyCode::DOP => "DOP",
            CurrencyCode::DZD => "DZD",
            CurrencyCode::EGP => "EGP",
            CurrencyCode::ERN => "ERN",
            CurrencyCode::ETB => "ETB",
            CurrencyCode::EUR => "EUR",
            CurrencyCode::FJD => "FJD",
            CurrencyCode::FKP => "FKP",
            CurrencyCode::GBP => "GBP",
            CurrencyCode::GEL => "GEL",
            CurrencyCode::GHS => "GHS",
            CurrencyCode::GIP => "GIP",
            CurrencyCode::GMD => "GMD",
            CurrencyCode::GNF => "GNF",
            CurrencyCode::GTQ => "GTQ",
            CurrencyCode::GYD => "GYD",
            CurrencyCode::HKD => "HKD",
            CurrencyCode::HNL => "HNL",
            CurrencyCode::HTG => "HTG",
            CurrencyCode::HUF => "HUF",
            CurrencyCode::IDR => "IDR",
            CurrencyCode::ILS => "ILS",
            CurrencyCode::INR => "INR",
            CurrencyCode::IQD => "IQD",
            CurrencyCode::IRR => "IRR",
            CurrencyCode::ISK => "ISK",
            CurrencyCode::JMD => "JMD",
            CurrencyCode::JOD => "JOD",
            CurrencyCode::JPY => "JPY",
            CurrencyCode::KES => "KES",
            CurrencyCode::KGS => "KGS",
            CurrencyCode::KHR => "KHR",
            CurrencyCode::KMF => "KMF",
            CurrencyCode::KPW => "KPW",
            CurrencyCode::KRW => "KRW",
            CurrencyCode::KWD => "KWD",
            CurrencyCode::KYD => "KYD",
            CurrencyCode::KZT => "KZT",
            CurrencyCode::LAK => "LAK",
            CurrencyCode::LBP => "LBP",
            CurrencyCode::LKR => "LKR",
            CurrencyCode::LRD => "LRD",
            CurrencyCode::LSL => "LSL",
            CurrencyCode::LYD => "LYD",
            CurrencyCode::MAD => "MAD",
            CurrencyCode::MDL => "MDL",
            CurrencyCode::MGA => "MGA",
            CurrencyCode::MKD => "MKD",
            CurrencyCode::MMK => "MMK",
            CurrencyCode::MNT => "MNT",
            CurrencyCode::MOP => "MOP",
            CurrencyCode::MRU => "MRU",
            CurrencyCode::MUR => "MUR",
            CurrencyCode::MVR => "MVR",
            CurrencyCode::MWK => "MWK",
            CurrencyCode::MXN => "MXN",
            CurrencyCode::MXV => "MXV",
            CurrencyCode::MYR => "MYR",
            CurrencyCode::MZN => "MZN",
            CurrencyCode::NAD => "NAD",
            CurrencyCode::NGN => "NGN",
            CurrencyCode::NIO => "NIO",
            CurrencyCode::NOK => "NOK",
            CurrencyCode::NPR => "NPR",
            CurrencyCode::NZD => "NZD",
            CurrencyCode::OMR => "OMR",
            CurrencyCode::PAB => "PAB",
            CurrencyCode::PEN => "PEN",
            CurrencyCode::PGK => "PGK",
            CurrencyCode::PHP => "PHP",
            CurrencyCode::PKR => "PKR",
            CurrencyCode::PLN => "PLN",
            CurrencyCode::PYG => "PYG",
            CurrencyCode::QAR => "QAR",
            CurrencyCode::RON => "RON",
            CurrencyCode::RSD => "RSD",
            CurrencyCode::CNY => "CNY",
            CurrencyCode::RUB => "RUB",
            CurrencyCode::RWF => "RWF",
            CurrencyCode::SAR => "SAR",
            CurrencyCode::SBD => "SBD",
            CurrencyCode::SCR => "SCR",
            CurrencyCode::SDG => "SDG",
            CurrencyCode::SEK => "SEK",
            CurrencyCode::SGD => "SGD",
            CurrencyCode::SHP => "SHP",
            CurrencyCode::SLE => "SLE",
            CurrencyCode::SLL => "SLL",
            CurrencyCode::SOS => "SOS",
            CurrencyCode::SRD => "SRD",
            CurrencyCode::SSP => "SSP",
            CurrencyCode::STN => "STN",
            CurrencyCode::SVC => "SVC",
            CurrencyCode::SYP => "SYP",
            CurrencyCode::SZL => "SZL",
            CurrencyCode::THB => "THB",
            CurrencyCode::TJS => "TJS",
            CurrencyCode::TMT => "TMT",
            CurrencyCode::TND => "TND",
            CurrencyCode::TOP => "TOP",
            CurrencyCode::TRY => "TRY",
            CurrencyCode::TTD => "TTD",
            CurrencyCode::TWD => "TWD",
            CurrencyCode::TZS => "TZS",
            CurrencyCode::UAH => "UAH",
            CurrencyCode::UGX => "UGX",
            CurrencyCode::USD => "USD",
            CurrencyCode::USN => "USN",
            CurrencyCode::UYI => "UYI",
            CurrencyCode::UYU => "UYU",
            CurrencyCode::UYW => "UYW",
            CurrencyCode::UZS => "UZS",
            CurrencyCode::VED => "VED",
            CurrencyCode::VES => "VES",
            CurrencyCode::VND => "VND",
            CurrencyCode::VUV => "VUV",
            CurrencyCode::WST => "WST",
            CurrencyCode::XAF => "XAF",
            CurrencyCode::XAG => "XAG",
            CurrencyCode::XAU => "XAU",
            CurrencyCode::XBA => "XBA",
            CurrencyCode::XBB => "XBB",
            CurrencyCode::XBC => "XBC",
            CurrencyCode::XBD => "XBD",
            CurrencyCode::XCD => "XCD",
            CurrencyCode::XDR => "XDR",
            CurrencyCode::XOF => "XOF",
            CurrencyCode::XPD => "XPD",
            CurrencyCode::XPF => "XPF",
            CurrencyCode::XPT => "XPT",
            CurrencyCode::XSU => "XSU",
            CurrencyCode::XTS => "XTS",
            CurrencyCode::XUA => "XUA",
            CurrencyCode::XXX => "XXX",
            CurrencyCode::YER => "YER",
            CurrencyCode::ZAR => "ZAR",
            CurrencyCode::ZMW => "ZMW",
            CurrencyCode::ZWL => "ZWL",
        }
    }
}

impl From<&CurrencyCode> for &'static str {
    fn from(val: &CurrencyCode) -> Self {
        (*val).into()
    }
}
