use chrono::DateTime;
use chrono::{NaiveDate, TimeZone, Utc};
use juniper::{GraphQLScalarValue, ScalarValue, Value};
use serde::{de, Deserialize, Serialize};
use std::fmt::{self, Display};
use std::str::FromStr;
use strum::Display;
use strum::EnumString;
use thiserror::Error;
use uuid::Uuid;
use yew::{html, Callback, Html, MouseEvent};

use crate::route::AppRoute;

use super::{CreateRoute, EditRoute, MetadataTable};

pub const DOI_DOMAIN: &str = "https://doi.org/";
pub const ORCID_DOMAIN: &str = "https://orcid.org/";
pub const ROR_DOMAIN: &str = "https://ror.org/";

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, EnumString, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkField {
    #[strum(serialize = "ID")]
    WorkId,
    #[strum(serialize = "Type")]
    WorkType,
    WorkStatus,
    #[strum(serialize = "Title")]
    #[default]
    FullTitle,
    #[strum(serialize = "ShortTitle")]
    Title,
    Subtitle,
    Reference,
    Edition,
    #[strum(serialize = "DOI")]
    Doi,
    PublicationDate,
    WithdrawnDate,
    Place,
    PageCount,
    PageBreakdown,
    ImageCount,
    TableCount,
    AudioCount,
    VideoCount,
    License,
    CopyrightHolder,
    LandingPage,
    #[strum(serialize = "LCCN")]
    Lccn,
    #[strum(serialize = "OCLC")]
    Oclc,
    ShortAbstract,
    LongAbstract,
    GeneralNote,
    BibliographyNote,
    #[strum(serialize = "TOC")]
    Toc,
    #[strum(serialize = "CoverURL")]
    CoverUrl,
    CoverCaption,
    CreatedAt,
    UpdatedAt,
    FirstPage,
    LastPage,
    PageInterval,
    UpdatedAtWithRelations,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkOrderBy {
    pub field: WorkField,
    pub direction: Direction,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WorkWithRelations {
    pub work_id: Uuid,
    pub work_type: WorkType,
    pub work_status: WorkStatus,
    pub full_title: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub reference: Option<String>,
    pub edition: Option<i32>,
    pub doi: Option<Doi>,
    pub publication_date: Option<String>,
    pub withdrawn_date: Option<String>,
    pub place: Option<String>,
    pub page_count: Option<i32>,
    pub page_breakdown: Option<String>,
    pub image_count: Option<i32>,
    pub table_count: Option<i32>,
    pub audio_count: Option<i32>,
    pub video_count: Option<i32>,
    pub license: Option<String>,
    pub copyright_holder: Option<String>,
    pub landing_page: Option<String>,
    pub lccn: Option<String>,
    pub oclc: Option<String>,
    pub short_abstract: Option<String>,
    pub long_abstract: Option<String>,
    pub general_note: Option<String>,
    pub bibliography_note: Option<String>,
    pub toc: Option<String>,
    pub cover_url: Option<String>,
    pub cover_caption: Option<String>,
    pub updated_at: Timestamp,
    pub first_page: Option<String>,
    pub last_page: Option<String>,
    pub page_interval: Option<String>,
    pub contributions: Option<Vec<Contribution>>,
    pub publications: Option<Vec<Publication>>,
    pub languages: Option<Vec<Language>>,
    pub fundings: Option<Vec<FundingWithInstitution>>,
    pub subjects: Option<Vec<Subject>>,
    pub issues: Option<Vec<IssueWithSeries>>,
    pub imprint: ImprintWithPublisher,
    pub relations: Option<Vec<WorkRelationWithRelatedWork>>,
    pub references: Option<Vec<Reference>>,
}

impl WorkWithRelations {
    pub fn compile_fulltitle(&self) -> String {
        if let Some(subtitle) = &self.subtitle.clone() {
            format!("{}: {}", self.title, subtitle)
        } else {
            self.title.to_string()
        }
    }

    pub fn compile_page_interval(&self) -> Option<String> {
        if let (Some(first), Some(last)) = (&self.first_page.clone(), &self.last_page.clone()) {
            Some(format!("{first}–{last}"))
        } else {
            None
        }
    }

    pub fn publisher(&self) -> String {
        if let Some(short_name) = &self.imprint.publisher.publisher_shortname.clone() {
            short_name.to_string()
        } else {
            self.imprint.publisher.publisher_name.to_string()
        }
    }
}

impl CreateRoute for WorkWithRelations {
    fn create_route() -> AppRoute {
        AppRoute::None
    }
}

impl EditRoute for WorkWithRelations {
    fn edit_route(&self) -> AppRoute {
        let work_id = self.work_id;
        AppRoute::BookDetail { book_id: work_id }
    }
}

impl MetadataTable for WorkWithRelations {
    fn as_table_row(&self, callback: Callback<MouseEvent>) -> Html {
        let doi = self.doi.as_ref().map(|s| s.to_string()).unwrap_or_default();
        let book_id = format!("/books/{}", self.work_id.clone());
        let book_name = self.full_title.clone();
        let imprint_name = self.imprint.imprint_name.clone();
        let contributors = match self.contributions.clone() {
            Some(w) => {
                let c = w
                    .clone()
                    .iter()
                    .map(|c| c.full_name.clone())
                    .collect::<Vec<_>>()
                    .join(", ");
                c
            }
            None => Default::default(),
        };
        let long_abstract = match self.long_abstract.clone() {
            Some(a) => a.to_owned(),
            None => Default::default(),
        };
        html! {
            <section class="flex flex-col md:flex-row gap-6 md:gap-9 lg:gap-11 py-5 lg:py-10 px-5 bg-white dark:bg-gray-700 rounded-md shadow-lg max-w-full text-justify" onclick={ callback }>
                <img alt="Book cover placeholder" sizes="25vw" class="object-cover h-48 cursor-pointer " role="link" tabindex="0" aria-label="Visit book page"
                    src= {self.cover_url.clone()} />
                <div>
                    <div class="pb-0.5 text-header text-lg"> { imprint_name } </div>
                    <h3 class="uppercase text-xl font-bold">
                        <a href={ book_id }> { book_name } </a>
                    </h3>
                    <ul class="text-l mt-2 mb-7 bullet-separated" role="list">
                        <li><span class="inline-block">{ contributors }</span></li>
                    </ul>
                    <div></div>
                    <div class="line-clamp-4">{ long_abstract }</div>
                </div>
            </section>
        }
    }
}

/// A specialised result type for returning Thoth data
pub type ThothResult<T> = std::result::Result<T, ThothError>;

impl Doi {
    pub fn to_lowercase_string(&self) -> String {
        self.0.to_lowercase()
    }
}

impl FromStr for Doi {
    type Err = ThothError;

    fn from_str(input: &str) -> ThothResult<Doi> {
        use lazy_static::lazy_static;
        use regex::Regex;
        lazy_static! {
            static ref RE: Regex = Regex::new(
            // ^    = beginning of string
            // (?:) = non-capturing group
            // i    = case-insensitive flag
            // $    = end of string
            // Matches strings of format "[[http[s]://][www.][dx.]doi.org/]10.XXX/XXX"
            // and captures the identifier segment starting with the "10." directory indicator
            // Corresponds to database constraints although regex syntax differs slightly
            // (e.g. `;()/` do not need to be escaped here)
            r"^(?i:(?:https?://)?(?:www\.)?(?:dx\.)?doi\.org/)?(10\.\d{4,9}/[-._;()\/:a-zA-Z0-9<>+\[\]]+$)").unwrap();
        }
        if input.is_empty() {
            Err(ThothError::DoiEmptyError)
        } else if let Some(matches) = RE.captures(input) {
            // The 0th capture always corresponds to the entire match
            if let Some(identifier) = matches.get(1) {
                let standardised = format!("{}{}", DOI_DOMAIN, identifier.as_str());
                let doi: Doi = Doi(standardised);
                Ok(doi)
            } else {
                Err(ThothError::DoiParseError(input.to_string()))
            }
        } else {
            Err(ThothError::DoiParseError(input.to_string()))
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Publication {
    pub publication_id: Uuid,
    pub publication_type: PublicationType,
    pub work_id: Uuid,
    pub isbn: Option<Isbn>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub width_mm: Option<f64>,
    pub width_in: Option<f64>,
    pub height_mm: Option<f64>,
    pub height_in: Option<f64>,
    pub depth_mm: Option<f64>,
    pub depth_in: Option<f64>,
    pub weight_g: Option<f64>,
    pub weight_oz: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize, EnumString, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PublicationType {
    #[cfg_attr(feature = "backend", db_rename = "Paperback")]
    #[default]
    Paperback,
    #[cfg_attr(feature = "backend", db_rename = "Hardback")]
    Hardback,
    #[cfg_attr(feature = "backend", db_rename = "PDF")]
    #[strum(serialize = "PDF")]
    Pdf,
    #[cfg_attr(feature = "backend", db_rename = "HTML")]
    #[strum(serialize = "HTML")]
    Html,
    #[cfg_attr(feature = "backend", db_rename = "XML")]
    #[strum(serialize = "XML")]
    Xml,
    #[cfg_attr(feature = "backend", db_rename = "Epub")]
    Epub,
    #[cfg_attr(feature = "backend", db_rename = "Mobi")]
    Mobi,
    #[cfg_attr(feature = "backend", db_rename = "AZW3")]
    #[strum(serialize = "AZW3")]
    Azw3,
    #[cfg_attr(feature = "backend", db_rename = "DOCX")]
    #[strum(serialize = "DOCX")]
    Docx,
    #[cfg_attr(feature = "backend", db_rename = "FictionBook")]
    FictionBook,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    pub language_id: Uuid,
    pub work_id: Uuid,
    pub language_code: LanguageCode,
    pub language_relation: LanguageRelation,
    pub main_language: bool,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize, EnumString, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "title_case")]
pub enum LanguageRelation {
    #[default]
    Original,
    #[cfg_attr(feature = "backend", db_rename = "translated-from")]
    TranslatedFrom,
    #[cfg_attr(feature = "backend", db_rename = "translated-into")]
    TranslatedInto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SubjectTypeValues {
    pub name: SubjectType,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize, EnumString, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "UPPERCASE")]
pub enum LanguageCode {
    Aar,
    Abk,
    Ace,
    Ach,
    Ada,
    Ady,
    Afa,
    Afh,
    Afr,
    Ain,
    Aka,
    Akk,
    Alb,
    Ale,
    Alg,
    Alt,
    Amh,
    Ang,
    Anp,
    Apa,
    Ara,
    Arc,
    Arg,
    Arm,
    Arn,
    Arp,
    Art,
    Arw,
    Asm,
    Ast,
    Ath,
    Aus,
    Ava,
    Ave,
    Awa,
    Aym,
    Aze,
    Bad,
    Bai,
    Bak,
    Bal,
    Bam,
    Ban,
    Baq,
    Bas,
    Bat,
    Bej,
    Bel,
    Bem,
    Ben,
    Ber,
    Bho,
    Bih,
    Bik,
    Bin,
    Bis,
    Bla,
    Bnt,
    Bos,
    Bra,
    Bre,
    Btk,
    Bua,
    Bug,
    Bul,
    Bur,
    Byn,
    Cad,
    Cai,
    Car,
    Cat,
    Cau,
    Ceb,
    Cel,
    Cha,
    Chb,
    Che,
    Chg,
    Chi,
    Chk,
    Chm,
    Chn,
    Cho,
    Chp,
    Chr,
    Chu,
    Chv,
    Chy,
    Cmc,
    Cnr,
    Cop,
    Cor,
    Cos,
    Cpe,
    Cpf,
    Cpp,
    Cre,
    Crh,
    Crp,
    Csb,
    Cus,
    Cze,
    Dak,
    Dan,
    Dar,
    Day,
    Del,
    Den,
    Dgr,
    Din,
    Div,
    Doi,
    Dra,
    Dsb,
    Dua,
    Dum,
    Dut,
    Dyu,
    Dzo,
    Efi,
    Egy,
    Eka,
    Elx,
    #[default]
    Eng,
    Enm,
    Epo,
    Est,
    Ewe,
    Ewo,
    Fan,
    Fao,
    Fat,
    Fij,
    Fil,
    Fin,
    Fiu,
    Fon,
    Fre,
    Frm,
    Fro,
    Frr,
    Frs,
    Fry,
    Ful,
    Fur,
    Gaa,
    Gay,
    Gba,
    Gem,
    Geo,
    Ger,
    Gez,
    Gil,
    Gla,
    Gle,
    Glg,
    Glv,
    Gmh,
    Goh,
    Gon,
    Gor,
    Got,
    Grb,
    Grc,
    Gre,
    Grn,
    Gsw,
    Guj,
    Gwi,
    Hai,
    Hat,
    Hau,
    Haw,
    Heb,
    Her,
    Hil,
    Him,
    Hin,
    Hit,
    Hmn,
    Hmo,
    Hrv,
    Hsb,
    Hun,
    Hup,
    Iba,
    Ibo,
    Ice,
    Ido,
    Iii,
    Ijo,
    Iku,
    Ile,
    Ilo,
    Ina,
    Inc,
    Ind,
    Ine,
    Inh,
    Ipk,
    Ira,
    Iro,
    Ita,
    Jav,
    Jbo,
    Jpn,
    Jpr,
    Jrb,
    Kaa,
    Kab,
    Kac,
    Kal,
    Kam,
    Kan,
    Kar,
    Kas,
    Kau,
    Kaw,
    Kaz,
    Kbd,
    Kha,
    Khi,
    Khm,
    Kho,
    Kik,
    Kin,
    Kir,
    Kmb,
    Kok,
    Kom,
    Kon,
    Kor,
    Kos,
    Kpe,
    Krc,
    Krl,
    Kro,
    Kru,
    Kua,
    Kum,
    Kur,
    Kut,
    Lad,
    Lah,
    Lam,
    Lao,
    Lat,
    Lav,
    Lez,
    Lim,
    Lin,
    Lit,
    Lol,
    Loz,
    Ltz,
    Lua,
    Lub,
    Lug,
    Lui,
    Lun,
    Luo,
    Lus,
    Mac,
    Mad,
    Mag,
    Mah,
    Mai,
    Mak,
    Mal,
    Man,
    Mao,
    Map,
    Mar,
    Mas,
    May,
    Mdf,
    Mdr,
    Men,
    Mga,
    Mic,
    Min,
    Mis,
    Mkh,
    Mlg,
    Mlt,
    Mnc,
    Mni,
    Mno,
    Moh,
    Mon,
    Mos,
    Mul,
    Mun,
    Mus,
    Mwl,
    Mwr,
    Myn,
    Myv,
    Nah,
    Nai,
    Nap,
    Nau,
    Nav,
    Nbl,
    Nde,
    Ndo,
    Nds,
    Nep,
    New,
    Nia,
    Nic,
    Niu,
    Nno,
    Nob,
    Nog,
    Non,
    Nor,
    Nqo,
    Nso,
    Nub,
    Nwc,
    Nya,
    Nym,
    Nyn,
    Nyo,
    Nzi,
    Oci,
    Oji,
    Ori,
    Orm,
    Osa,
    Oss,
    Ota,
    Oto,
    Paa,
    Pag,
    Pal,
    Pam,
    Pan,
    Pap,
    Pau,
    Peo,
    Per,
    Phi,
    Phn,
    Pli,
    Pol,
    Pon,
    Por,
    Pra,
    Pro,
    Pus,
    Qaa,
    Que,
    Raj,
    Rap,
    Rar,
    Roa,
    Roh,
    Rom,
    Rum,
    Run,
    Rup,
    Rus,
    Sad,
    Sag,
    Sah,
    Sai,
    Sal,
    Sam,
    San,
    Sas,
    Sat,
    Scn,
    Sco,
    Sel,
    Sem,
    Sga,
    Sgn,
    Shn,
    Sid,
    Sin,
    Sio,
    Sit,
    Sla,
    Slo,
    Slv,
    Sma,
    Sme,
    Smi,
    Smj,
    Smn,
    Smo,
    Sms,
    Sna,
    Snd,
    Snk,
    Sog,
    Som,
    Son,
    Sot,
    Spa,
    Srd,
    Srn,
    Srp,
    Srr,
    Ssa,
    Ssw,
    Suk,
    Sun,
    Sus,
    Sux,
    Swa,
    Swe,
    Syc,
    Syr,
    Tah,
    Tai,
    Tam,
    Tat,
    Tel,
    Tem,
    Ter,
    Tet,
    Tgk,
    Tgl,
    Tha,
    Tib,
    Tig,
    Tir,
    Tiv,
    Tkl,
    Tlh,
    Tli,
    Tmh,
    Tog,
    Ton,
    Tpi,
    Tsi,
    Tsn,
    Tso,
    Tuk,
    Tum,
    Tup,
    Tur,
    Tut,
    Tvl,
    Twi,
    Tyv,
    Udm,
    Uga,
    Uig,
    Ukr,
    Umb,
    Und,
    Urd,
    Uzb,
    Vai,
    Ven,
    Vie,
    Vol,
    Vot,
    Wak,
    Wal,
    War,
    Was,
    Wel,
    Wen,
    Wln,
    Wol,
    Xal,
    Xho,
    Yao,
    Yap,
    Yid,
    Yor,
    Ypk,
    Zap,
    Zbl,
    Zen,
    Zgh,
    Zha,
    Znd,
    Zul,
    Zun,
    Zxx,
    Zza,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    pub subject_id: Uuid,
    pub work_id: Uuid,
    pub subject_type: SubjectType,
    pub subject_code: String,
    pub subject_ordinal: i32,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FundingWithInstitution {
    pub funding_id: Uuid,
    pub work_id: Uuid,
    pub institution_id: Uuid,
    pub program: Option<String>,
    pub project_name: Option<String>,
    pub project_shortname: Option<String>,
    pub grant_number: Option<String>,
    pub jurisdiction: Option<String>,
    pub institution: Institution,
}

#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Deserialize,
    Serialize,
    EnumString,
    Display,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubjectType {
    #[strum(serialize = "BIC")]
    Bic,
    #[strum(serialize = "BISAC")]
    Bisac,
    Thema,
    #[strum(serialize = "LCC")]
    Lcc,
    Custom,
    #[default]
    Keyword,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SeriesTypeValues {
    pub name: SeriesType,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Institution {
    pub institution_id: Uuid,
    pub institution_name: String,
    pub institution_doi: Option<Doi>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub ror: Option<Ror>,
    pub country_code: Option<CountryCode>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Ror(String);

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, EnumString, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CountryCode {
    #[strum(serialize = "Afghanistan")]
    Afg,
    #[strum(serialize = "Åland Islands")]
    Ala,
    #[strum(serialize = "Albania")]
    Alb,
    #[strum(serialize = "Algeria")]
    Dza,
    #[strum(serialize = "American Samoa")]
    Asm,
    #[strum(serialize = "Andorra")]
    And,
    #[strum(serialize = "Angola")]
    Ago,
    #[strum(serialize = "Anguilla")]
    Aia,
    #[strum(serialize = "Antarctica")]
    Ata,
    #[strum(serialize = "Antigua and Barbuda")]
    Atg,
    #[strum(serialize = "Argentina")]
    Arg,
    #[strum(serialize = "Armenia")]
    Arm,
    #[strum(serialize = "Aruba")]
    Abw,
    #[strum(serialize = "Australia")]
    Aus,
    #[strum(serialize = "Austria")]
    Aut,
    #[strum(serialize = "Azerbaijan")]
    Aze,
    #[strum(serialize = "Bahamas")]
    Bhs,
    #[strum(serialize = "Bahrain")]
    Bhr,
    #[strum(serialize = "Bangladesh")]
    Bgd,
    #[strum(serialize = "Barbados")]
    Brb,
    #[strum(serialize = "Belarus")]
    Blr,
    #[strum(serialize = "Belgium")]
    Bel,
    #[strum(serialize = "Belize")]
    Blz,
    #[strum(serialize = "Benin")]
    Ben,
    #[strum(serialize = "Bermuda")]
    Bmu,
    #[strum(serialize = "Bhutan")]
    Btn,
    #[strum(serialize = "Bolivia")]
    Bol,
    #[strum(serialize = "Bonaire, Sint Eustatius and Saba")]
    Bes,
    #[strum(serialize = "Bosnia and Herzegovina")]
    Bih,
    #[strum(serialize = "Botswana")]
    Bwa,
    #[strum(serialize = "Bouvet Island")]
    Bvt,
    #[strum(serialize = "Brazil")]
    Bra,
    #[strum(serialize = "British Indian Ocean Territory")]
    Iot,
    #[strum(serialize = "Brunei")]
    Brn,
    #[strum(serialize = "Bulgaria")]
    Bgr,
    #[strum(serialize = "Burkina Faso")]
    Bfa,
    #[strum(serialize = "Burundi")]
    Bdi,
    #[strum(serialize = "Cabo Verde")]
    Cpv,
    #[strum(serialize = "Cambodia")]
    Khm,
    #[strum(serialize = "Cameroon")]
    Cmr,
    #[strum(serialize = "Canada")]
    Can,
    #[strum(serialize = "Cayman Islands")]
    Cym,
    #[strum(serialize = "Central African Republic")]
    Caf,
    #[strum(serialize = "Chad")]
    Tcd,
    #[strum(serialize = "Chile")]
    Chl,
    #[strum(serialize = "China")]
    Chn,
    #[strum(serialize = "Christmas Island")]
    Cxr,
    #[strum(serialize = "Cocos (Keeling) Islands")]
    Cck,
    #[strum(serialize = "Colombia")]
    Col,
    #[strum(serialize = "Comoros")]
    Com,
    #[strum(serialize = "Cook Islands")]
    Cok,
    #[strum(serialize = "Costa Rica")]
    Cri,
    #[strum(serialize = "Côte d'Ivoire")]
    Civ,
    #[strum(serialize = "Croatia")]
    Hrv,
    #[strum(serialize = "Cuba")]
    Cub,
    #[strum(serialize = "Curaçao")]
    Cuw,
    #[strum(serialize = "Cyprus")]
    Cyp,
    #[strum(serialize = "Czechia")]
    Cze,
    #[strum(serialize = "Democratic Republic of the Congo")]
    Cod,
    #[strum(serialize = "Denmark")]
    Dnk,
    #[strum(serialize = "Djibouti")]
    Dji,
    #[strum(serialize = "Dominica")]
    Dma,
    #[strum(serialize = "Dominican Republic")]
    Dom,
    #[strum(serialize = "Ecuador")]
    Ecu,
    #[strum(serialize = "Egypt")]
    Egy,
    #[strum(serialize = "El Salvador")]
    Slv,
    #[strum(serialize = "Equatorial Guinea")]
    Gnq,
    #[strum(serialize = "Eritrea")]
    Eri,
    #[strum(serialize = "Estonia")]
    Est,
    #[strum(serialize = "Eswatini")]
    Swz,
    #[strum(serialize = "Ethiopia")]
    Eth,
    #[strum(serialize = "Falkland Islands")]
    Flk,
    #[strum(serialize = "Faroe Islands")]
    Fro,
    #[strum(serialize = "Fiji")]
    Fji,
    #[strum(serialize = "Finland")]
    Fin,
    #[strum(serialize = "France")]
    Fra,
    #[strum(serialize = "French Guiana")]
    Guf,
    #[strum(serialize = "French Polynesia")]
    Pyf,
    #[strum(serialize = "French Southern Territories")]
    Atf,
    #[strum(serialize = "Gabon")]
    Gab,
    #[strum(serialize = "Gambia")]
    Gmb,
    #[strum(serialize = "Georgia")]
    Geo,
    #[strum(serialize = "Germany")]
    Deu,
    #[strum(serialize = "Ghana")]
    Gha,
    #[strum(serialize = "Gibraltar")]
    Gib,
    #[strum(serialize = "Greece")]
    Grc,
    #[strum(serialize = "Greenland")]
    Grl,
    #[strum(serialize = "Grenada")]
    Grd,
    #[strum(serialize = "Guadeloupe")]
    Glp,
    #[strum(serialize = "Guam")]
    Gum,
    #[strum(serialize = "Guatemala")]
    Gtm,
    #[strum(serialize = "Guernsey")]
    Ggy,
    #[strum(serialize = "Guinea")]
    Gin,
    #[strum(serialize = "Guinea-Bissau")]
    Gnb,
    #[strum(serialize = "Guyana")]
    Guy,
    #[strum(serialize = "Haiti")]
    Hti,
    #[strum(serialize = "Heard Island and McDonald Islands")]
    Hmd,
    #[strum(serialize = "Honduras")]
    Hnd,
    #[strum(serialize = "Hong Kong")]
    Hkg,
    #[strum(serialize = "Hungary")]
    Hun,
    #[strum(serialize = "Iceland")]
    Isl,
    #[strum(serialize = "India")]
    Ind,
    #[strum(serialize = "Indonesia")]
    Idn,
    #[strum(serialize = "Iran")]
    Irn,
    #[strum(serialize = "Iraq")]
    Irq,
    #[strum(serialize = "Ireland")]
    Irl,
    #[strum(serialize = "Isle of Man")]
    Imn,
    #[strum(serialize = "Israel")]
    Isr,
    #[strum(serialize = "Italy")]
    Ita,
    #[strum(serialize = "Jamaica")]
    Jam,
    #[strum(serialize = "Japan")]
    Jpn,
    #[strum(serialize = "Jersey")]
    Jey,
    #[strum(serialize = "Jordan")]
    Jor,
    #[strum(serialize = "Kazakhstan")]
    Kaz,
    #[strum(serialize = "Kenya")]
    Ken,
    #[strum(serialize = "Kiribati")]
    Kir,
    #[strum(serialize = "Kuwait")]
    Kwt,
    #[strum(serialize = "Kyrgyzstan")]
    Kgz,
    #[strum(serialize = "Laos")]
    Lao,
    #[strum(serialize = "Latvia")]
    Lva,
    #[strum(serialize = "Lebanon")]
    Lbn,
    #[strum(serialize = "Lesotho")]
    Lso,
    #[strum(serialize = "Liberia")]
    Lbr,
    #[strum(serialize = "Libya")]
    Lby,
    #[strum(serialize = "Liechtenstein")]
    Lie,
    #[strum(serialize = "Lithuania")]
    Ltu,
    #[strum(serialize = "Luxembourg")]
    Lux,
    #[strum(serialize = "Macao")]
    Mac,
    #[strum(serialize = "Madagascar")]
    Mdg,
    #[strum(serialize = "Malawi")]
    Mwi,
    #[strum(serialize = "Malaysia")]
    Mys,
    #[strum(serialize = "Maldives")]
    Mdv,
    #[strum(serialize = "Mali")]
    Mli,
    #[strum(serialize = "Malta")]
    Mlt,
    #[strum(serialize = "Marshall Islands")]
    Mhl,
    #[strum(serialize = "Martinique")]
    Mtq,
    #[strum(serialize = "Mauritania")]
    Mrt,
    #[strum(serialize = "Mauritius")]
    Mus,
    #[strum(serialize = "Mayotte")]
    Myt,
    #[strum(serialize = "Mexico")]
    Mex,
    #[strum(serialize = "Micronesia")]
    Fsm,
    #[strum(serialize = "Moldova")]
    Mda,
    #[strum(serialize = "Monaco")]
    Mco,
    #[strum(serialize = "Mongolia")]
    Mng,
    #[strum(serialize = "Montenegro")]
    Mne,
    #[strum(serialize = "Montserrat")]
    Msr,
    #[strum(serialize = "Morocco")]
    Mar,
    #[strum(serialize = "Mozambique")]
    Moz,
    #[strum(serialize = "Myanmar")]
    Mmr,
    #[strum(serialize = "Namibia")]
    Nam,
    #[strum(serialize = "Nauru")]
    Nru,
    #[strum(serialize = "Nepal")]
    Npl,
    #[strum(serialize = "Netherlands")]
    Nld,
    #[strum(serialize = "New Caledonia")]
    Ncl,
    #[strum(serialize = "New Zealand")]
    Nzl,
    #[strum(serialize = "Nicaragua")]
    Nic,
    #[strum(serialize = "Niger")]
    Ner,
    #[strum(serialize = "Nigeria")]
    Nga,
    #[strum(serialize = "Niue")]
    Niu,
    #[strum(serialize = "Norfolk Island")]
    Nfk,
    #[strum(serialize = "North Korea")]
    Prk,
    #[strum(serialize = "North Macedonia")]
    Mkd,
    #[strum(serialize = "Northern Mariana Islands")]
    Mnp,
    #[strum(serialize = "Norway")]
    Nor,
    #[strum(serialize = "Oman")]
    Omn,
    #[strum(serialize = "Pakistan")]
    Pak,
    #[strum(serialize = "Palau")]
    Plw,
    #[strum(serialize = "Palestine")]
    Pse,
    #[strum(serialize = "Panama")]
    Pan,
    #[strum(serialize = "Papua New Guinea")]
    Png,
    #[strum(serialize = "Paraguay")]
    Pry,
    #[strum(serialize = "Peru")]
    Per,
    #[strum(serialize = "Philippines")]
    Phl,
    #[strum(serialize = "Pitcairn")]
    Pcn,
    #[strum(serialize = "Poland")]
    Pol,
    #[strum(serialize = "Portugal")]
    Prt,
    #[strum(serialize = "Puerto Rico")]
    Pri,
    #[strum(serialize = "Qatar")]
    Qat,
    #[strum(serialize = "Republic of the Congo")]
    Cog,
    #[strum(serialize = "Réunion")]
    Reu,
    #[strum(serialize = "Romania")]
    Rou,
    #[strum(serialize = "Russia")]
    Rus,
    #[strum(serialize = "Rwanda")]
    Rwa,
    #[strum(serialize = "Saint Barthélemy")]
    Blm,
    #[strum(serialize = "Saint Helena, Ascension and Tristan da Cunha")]
    Shn,
    #[strum(serialize = "Saint Kitts and Nevis")]
    Kna,
    #[strum(serialize = "Saint Lucia")]
    Lca,
    #[strum(serialize = "Saint Martin")]
    Maf,
    #[strum(serialize = "Saint Pierre and Miquelon")]
    Spm,
    #[strum(serialize = "Saint Vincent and the Grenadines")]
    Vct,
    #[strum(serialize = "Samoa")]
    Wsm,
    #[strum(serialize = "San Marino")]
    Smr,
    #[strum(serialize = "Sao Tome and Principe")]
    Stp,
    #[strum(serialize = "Saudi Arabia")]
    Sau,
    #[strum(serialize = "Senegal")]
    Sen,
    #[strum(serialize = "Serbia")]
    Srb,
    #[strum(serialize = "Seychelles")]
    Syc,
    #[strum(serialize = "Sierra Leone")]
    Sle,
    #[strum(serialize = "Singapore")]
    Sgp,
    #[strum(serialize = "Sint Maarten")]
    Sxm,
    #[strum(serialize = "Slovakia")]
    Svk,
    #[strum(serialize = "Slovenia")]
    Svn,
    #[strum(serialize = "Solomon Islands")]
    Slb,
    #[strum(serialize = "Somalia")]
    Som,
    #[strum(serialize = "South Africa")]
    Zaf,
    #[strum(serialize = "South Georgia and the South Sandwich Islands")]
    Sgs,
    #[strum(serialize = "South Korea")]
    Kor,
    #[strum(serialize = "South Sudan")]
    Ssd,
    #[strum(serialize = "Spain")]
    Esp,
    #[strum(serialize = "Sri Lanka")]
    Lka,
    #[strum(serialize = "Sudan")]
    Sdn,
    #[strum(serialize = "Suriname")]
    Sur,
    #[strum(serialize = "Svalbard and Jan Mayen")]
    Sjm,
    #[strum(serialize = "Sweden")]
    Swe,
    #[strum(serialize = "Switzerland")]
    Che,
    #[strum(serialize = "Syria")]
    Syr,
    #[strum(serialize = "Taiwan")]
    Twn,
    #[strum(serialize = "Tajikistan")]
    Tjk,
    #[strum(serialize = "Tanzania")]
    Tza,
    #[strum(serialize = "Thailand")]
    Tha,
    #[strum(serialize = "Timor-Leste")]
    Tls,
    #[strum(serialize = "Togo")]
    Tgo,
    #[strum(serialize = "Tokelau")]
    Tkl,
    #[strum(serialize = "Tonga")]
    Ton,
    #[strum(serialize = "Trinidad and Tobago")]
    Tto,
    #[strum(serialize = "Tunisia")]
    Tun,
    #[strum(serialize = "Turkey")]
    Tur,
    #[strum(serialize = "Turkmenistan")]
    Tkm,
    #[strum(serialize = "Turks and Caicos Islands")]
    Tca,
    #[strum(serialize = "Tuvalu")]
    Tuv,
    #[strum(serialize = "Uganda")]
    Uga,
    #[strum(serialize = "Ukraine")]
    Ukr,
    #[strum(serialize = "United Arab Emirates")]
    Are,
    #[strum(serialize = "United Kingdom")]
    Gbr,
    #[strum(serialize = "United States Minor Outlying Islands")]
    Umi,
    #[strum(serialize = "United States of America")]
    Usa,
    #[strum(serialize = "Uruguay")]
    Ury,
    #[strum(serialize = "Uzbekistan")]
    Uzb,
    #[strum(serialize = "Vanuatu")]
    Vut,
    #[strum(serialize = "Vatican City")]
    Vat,
    #[strum(serialize = "Venezuela")]
    Ven,
    #[strum(serialize = "Viet Nam")]
    Vnm,
    #[strum(serialize = "Virgin Islands (British)")]
    Vgb,
    #[strum(serialize = "Virgin Islands (U.S.)")]
    Vir,
    #[strum(serialize = "Wallis and Futuna")]
    Wlf,
    #[strum(serialize = "Western Sahara")]
    Esh,
    #[strum(serialize = "Yemen")]
    Yem,
    #[strum(serialize = "Zambia")]
    Zmb,
    #[strum(serialize = "Zimbabwe")]
    Zwe,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct IssueWithSeries {
    pub issue_id: Uuid,
    pub work_id: Uuid,
    pub series_id: Uuid,
    pub issue_ordinal: i32,
    pub series: SeriesWithImprint,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SeriesWithImprint {
    pub series_id: Uuid,
    pub series_type: SeriesType,
    pub series_name: String,
    pub issn_print: Option<String>,
    pub issn_digital: Option<String>,
    pub series_url: Option<String>,
    pub series_description: Option<String>,
    pub series_cfp_url: Option<String>,
    pub updated_at: Timestamp,
    pub imprint: ImprintWithPublisher,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize, EnumString, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "title_case")]
pub enum SeriesType {
    Journal,
    #[cfg_attr(feature = "backend", db_rename = "book-series")]
    #[default]
    BookSeries,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Variables {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub filter: Option<String>,
    pub order: Option<WorkOrderBy>,
    pub publishers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, juniper::GraphQLEnum)]
#[graphql(description = "Order in which to sort query results (ascending or descending)")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Direction {
    #[default]
    Asc,
    Desc,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Timestamp(DateTime<Utc>);

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ImprintWithPublisher {
    pub imprint_id: Uuid,
    pub imprint_name: String,
    pub imprint_url: Option<String>,
    pub crossmark_doi: Option<Doi>,
    pub updated_at: Timestamp,
    pub publisher: Publisher,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContributionTypeValues {
    pub name: ContributionType,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Publisher {
    pub publisher_id: Uuid,
    pub publisher_name: String,
    pub publisher_shortname: Option<String>,
    pub publisher_url: Option<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WorkRelationWithRelatedWork {
    pub work_relation_id: Uuid,
    pub relator_work_id: Uuid,
    pub related_work_id: Uuid,
    pub relation_type: RelationType,
    pub relation_ordinal: i32,
    pub related_work: Work,
}

#[derive(
    Debug, Clone, Default, Copy, PartialEq, Eq, Deserialize, Serialize, EnumString, Display,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "title_case")]
pub enum RelationType {
    Replaces,
    #[cfg_attr(feature = "backend", db_rename = "has-translation")]
    HasTranslation,
    #[cfg_attr(feature = "backend", db_rename = "has-part")]
    HasPart,
    #[cfg_attr(feature = "backend", db_rename = "has-child")]
    #[default]
    HasChild,
    #[cfg_attr(feature = "backend", db_rename = "is-replaced-by")]
    IsReplacedBy,
    #[cfg_attr(feature = "backend", db_rename = "is-translation-of")]
    IsTranslationOf,
    #[cfg_attr(feature = "backend", db_rename = "is-part-of")]
    IsPartOf,
    #[cfg_attr(feature = "backend", db_rename = "is-child-of")]
    IsChildOf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CountryCodeValues {
    pub name: CountryCode,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Work {
    pub work_id: Uuid,
    pub work_type: WorkType,
    pub work_status: WorkStatus,
    pub full_title: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub reference: Option<String>,
    pub edition: Option<i32>,
    pub imprint_id: Uuid,
    pub doi: Option<Doi>,
    pub publication_date: Option<NaiveDate>,
    pub withdrawn_date: Option<NaiveDate>,
    pub place: Option<String>,
    pub page_count: Option<i32>,
    pub page_breakdown: Option<String>,
    pub image_count: Option<i32>,
    pub table_count: Option<i32>,
    pub audio_count: Option<i32>,
    pub video_count: Option<i32>,
    pub license: Option<String>,
    pub copyright_holder: Option<String>,
    pub landing_page: Option<String>,
    pub lccn: Option<String>,
    pub oclc: Option<String>,
    pub short_abstract: Option<String>,
    pub long_abstract: Option<String>,
    pub general_note: Option<String>,
    pub bibliography_note: Option<String>,
    pub toc: Option<String>,
    pub cover_url: Option<String>,
    pub cover_caption: Option<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub first_page: Option<String>,
    pub last_page: Option<String>,
    pub page_interval: Option<String>,
    pub updated_at_with_relations: Timestamp,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize, EnumString, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "title_case")]
pub enum WorkType {
    #[cfg_attr(feature = "backend", db_rename = "book-chapter")]
    BookChapter,
    #[default]
    Monograph,
    #[cfg_attr(feature = "backend", db_rename = "edited-book")]
    EditedBook,
    Textbook,
    #[cfg_attr(feature = "backend", db_rename = "journal-issue")]
    JournalIssue,
    #[cfg_attr(feature = "backend", db_rename = "book-set")]
    BookSet,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Contribution {
    pub contribution_id: Uuid,
    pub work_id: Uuid,
    pub contributor_id: Uuid,
    pub contribution_type: ContributionType,
    pub main_contribution: bool,
    pub biography: Option<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub first_name: Option<String>,
    pub last_name: String,
    pub full_name: String,
    pub contribution_ordinal: i32,
    pub contributor: Contributor,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Reference {
    pub reference_id: Uuid,
    pub work_id: Uuid,
    pub reference_ordinal: i32,
    pub doi: Option<Doi>,
    pub unstructured_citation: Option<String>,
    pub issn: Option<String>,
    pub isbn: Option<Isbn>,
    pub journal_title: Option<String>,
    pub article_title: Option<String>,
    pub series_title: Option<String>,
    pub volume_title: Option<String>,
    pub edition: Option<i32>,
    pub author: Option<String>,
    pub volume: Option<String>,
    pub issue: Option<String>,
    pub first_page: Option<String>,
    pub component_number: Option<String>,
    pub standard_designator: Option<String>,
    pub standards_body_name: Option<String>,
    pub standards_body_acronym: Option<String>,
    pub url: Option<String>,
    pub publication_date: Option<NaiveDate>,
    pub retrieval_date: Option<NaiveDate>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RelationTypeValues {
    pub name: RelationType,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize, EnumString, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LocationPlatform {
    #[cfg_attr(feature = "backend", db_rename = "Project MUSE")]
    #[strum(serialize = "Project MUSE")]
    ProjectMuse,
    #[cfg_attr(feature = "backend", db_rename = "OAPEN")]
    #[strum(serialize = "OAPEN")]
    Oapen,
    #[cfg_attr(feature = "backend", db_rename = "DOAB")]
    #[strum(serialize = "DOAB")]
    Doab,
    #[cfg_attr(feature = "backend", db_rename = "JSTOR")]
    #[strum(serialize = "JSTOR")]
    Jstor,
    #[cfg_attr(feature = "backend", db_rename = "EBSCO Host")]
    #[strum(serialize = "EBSCO Host")]
    EbscoHost,
    #[cfg_attr(feature = "backend", db_rename = "OCLC KB")]
    #[strum(serialize = "OCLC KB")]
    OclcKb,
    #[cfg_attr(feature = "backend", db_rename = "ProQuest KB")]
    #[strum(serialize = "ProQuest KB")]
    ProquestKb,
    #[cfg_attr(feature = "backend", db_rename = "ProQuest ExLibris")]
    #[strum(serialize = "ProQuest ExLibris")]
    ProquestExlibris,
    #[cfg_attr(feature = "backend", db_rename = "EBSCO KB")]
    #[strum(serialize = "EBSCO KB")]
    EbscoKb,
    #[cfg_attr(feature = "backend", db_rename = "JISC KB")]
    #[strum(serialize = "JISC KB")]
    JiscKb,
    #[cfg_attr(feature = "backend", db_rename = "Google Books")]
    #[strum(serialize = "Google Books")]
    GoogleBooks,
    #[cfg_attr(feature = "backend", db_rename = "Internet Archive")]
    #[strum(serialize = "Internet Archive")]
    InternetArchive,
    #[cfg_attr(feature = "backend", db_rename = "ScienceOpen")]
    #[strum(serialize = "ScienceOpen")]
    ScienceOpen,
    #[cfg_attr(feature = "backend", db_rename = "SciELO Books")]
    #[strum(serialize = "SciELO Books")]
    ScieloBooks,
    #[cfg_attr(feature = "backend", db_rename = "Zenodo")]
    #[strum(serialize = "Zenodo")]
    Zenodo,
    #[cfg_attr(feature = "backend", db_rename = "Publisher Website")]
    #[strum(serialize = "Publisher Website")]
    PublisherWebsite,
    #[cfg_attr(feature = "backend", db_rename = "Other")]
    #[default]
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LanguageRelationValues {
    pub name: LanguageRelation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyCodeValues {
    pub name: CurrencyCode,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize, EnumString, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "UPPERCASE")]
pub enum CurrencyCode {
    Adp,
    Aed,
    Afa,
    Afn,
    Alk,
    All,
    Amd,
    Ang,
    Aoa,
    Aok,
    Aon,
    Aor,
    Ara,
    Arp,
    Ars,
    Ary,
    Ats,
    Aud,
    Awg,
    Aym,
    Azm,
    Azn,
    Bad,
    Bam,
    Bbd,
    Bdt,
    Bec,
    Bef,
    Bel,
    Bgj,
    Bgk,
    Bgl,
    Bgn,
    Bhd,
    Bif,
    Bmd,
    Bnd,
    Bob,
    Bop,
    Bov,
    Brb,
    Brc,
    Bre,
    Brl,
    Brn,
    Brr,
    Bsd,
    Btn,
    Buk,
    Bwp,
    Byb,
    Byn,
    Byr,
    Bzd,
    Cad,
    Cdf,
    Chc,
    Che,
    Chf,
    Chw,
    Clf,
    Clp,
    Cny,
    Cop,
    Cou,
    Crc,
    Csd,
    Csj,
    Csk,
    Cuc,
    Cup,
    Cve,
    Cyp,
    Czk,
    Ddm,
    Dem,
    Djf,
    Dkk,
    Dop,
    Dzd,
    Ecs,
    Ecv,
    Eek,
    Egp,
    Ern,
    Esa,
    Esb,
    Esp,
    Etb,
    Eur,
    Fim,
    Fjd,
    Fkp,
    Frf,
    #[default]
    Gbp,
    Gek,
    Gel,
    Ghc,
    Ghp,
    Ghs,
    Gip,
    Gmd,
    Gne,
    Gnf,
    Gns,
    Gqe,
    Grd,
    Gtq,
    Gwe,
    Gwp,
    Gyd,
    Hkd,
    Hnl,
    Hrd,
    Hrk,
    Htg,
    Huf,
    Idr,
    Iep,
    Ilp,
    Ilr,
    Ils,
    Inr,
    Iqd,
    Irr,
    Isj,
    Isk,
    Itl,
    Jmd,
    Jod,
    Jpy,
    Kes,
    Kgs,
    Khr,
    Kmf,
    Kpw,
    Krw,
    Kwd,
    Kyd,
    Kzt,
    Laj,
    Lak,
    Lbp,
    Lkr,
    Lrd,
    Lsl,
    Lsm,
    Ltl,
    Ltt,
    Luc,
    Luf,
    Lul,
    Lvl,
    Lvr,
    Lyd,
    Mad,
    Mdl,
    Mga,
    Mgf,
    Mkd,
    Mlf,
    Mmk,
    Mnt,
    Mop,
    Mro,
    Mru,
    Mtl,
    Mtp,
    Mur,
    Mvq,
    Mvr,
    Mwk,
    Mxn,
    Mxp,
    Mxv,
    Myr,
    Mze,
    Mzm,
    Mzn,
    Nad,
    Ngn,
    Nic,
    Nio,
    Nlg,
    Nok,
    Npr,
    Nzd,
    Omr,
    Pab,
    Peh,
    Pei,
    Pen,
    Pes,
    Pgk,
    Php,
    Pkr,
    Pln,
    Plz,
    Pte,
    Pyg,
    Qar,
    Rhd,
    Rok,
    Rol,
    Ron,
    Rsd,
    Rub,
    Rur,
    Rwf,
    Sar,
    Sbd,
    Scr,
    Sdd,
    Sdg,
    Sdp,
    Sek,
    Sgd,
    Shp,
    Sit,
    Skk,
    Sll,
    Sos,
    Srd,
    Srg,
    Ssp,
    Std,
    Stn,
    Sur,
    Svc,
    Syp,
    Szl,
    Thb,
    Tjr,
    Tjs,
    Tmm,
    Tmt,
    Tnd,
    Top,
    Tpe,
    Trl,
    Try,
    Ttd,
    Twd,
    Tzs,
    Uah,
    Uak,
    Ugs,
    Ugw,
    Ugx,
    Usd,
    Usn,
    Uss,
    Uyi,
    Uyn,
    Uyp,
    Uyu,
    Uyw,
    Uzs,
    Veb,
    Vef,
    Ves,
    Vnc,
    Vnd,
    Vuv,
    Wst,
    Xaf,
    Xag,
    Xau,
    Xba,
    Xbb,
    Xbc,
    Xbd,
    Xcd,
    Xdr,
    Xeu,
    Xfo,
    Xfu,
    Xof,
    Xpd,
    Xpf,
    Xpt,
    Xre,
    Xsu,
    Xts,
    Xua,
    Xxx,
    Ydd,
    Yer,
    Yud,
    Yum,
    Yun,
    Zal,
    Zar,
    Zmk,
    Zmw,
    Zrn,
    Zrz,
    Zwc,
    Zwd,
    Zwl,
    Zwn,
    Zwr,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LocationPlatformValues {
    pub name: LocationPlatform,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Contributor {
    pub contributor_id: Uuid,
    pub first_name: Option<String>,
    pub last_name: String,
    pub full_name: String,
    pub orcid: Option<Orcid>,
    pub website: Option<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LanguageCodeValues {
    pub name: LanguageCode,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Orcid(String);

impl Display for Orcid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Isbn(String);

impl Display for Isbn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(
    Debug, Clone, Default, Copy, PartialEq, Eq, Deserialize, Serialize, EnumString, Display,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "title_case")]
pub enum ContributionType {
    #[cfg_attr(feature = "backend", graphql(description = "Author of the work"))]
    #[default]
    Author,
    #[cfg_attr(feature = "backend", graphql(description = "Editor of the work"))]
    Editor,
    #[cfg_attr(feature = "backend", graphql(description = "Translator of the work"))]
    Translator,
    #[cfg_attr(
        feature = "backend",
        graphql(
            description = "Photographer when named as the primary creator of, eg, a book of photographs"
        )
    )]
    Photographer,
    #[cfg_attr(
        feature = "backend",
        graphql(
            description = "Artist when named as the creator of artwork which illustrates a work"
        )
    )]
    Illustrator,
    #[cfg_attr(
        feature = "backend",
        db_rename = "music-editor",
        graphql(
            description = "Person responsible for editing any piece of music referenced in the work"
        )
    )]
    MusicEditor,
    #[cfg_attr(
        feature = "backend",
        db_rename = "foreword-by",
        graphql(description = "Author of foreword")
    )]
    ForewordBy,
    #[cfg_attr(
        feature = "backend",
        db_rename = "introduction-by",
        graphql(description = "Author of introduction")
    )]
    IntroductionBy,
    #[cfg_attr(
        feature = "backend",
        db_rename = "afterword-by",
        graphql(description = "Author of afterword")
    )]
    AfterwordBy,
    #[cfg_attr(
        feature = "backend",
        db_rename = "preface-by",
        graphql(description = "Author of preface")
    )]
    PrefaceBy,
    #[cfg_attr(
        feature = "backend",
        db_rename = "software-by",
        graphql(description = "Writer of computer programs ancillary to the work")
    )]
    SoftwareBy,
    #[cfg_attr(
        feature = "backend",
        db_rename = "research-by",
        graphql(
            description = "Person responsible for performing research on which the work is based"
        )
    )]
    ResearchBy,
    #[cfg_attr(
        feature = "backend",
        db_rename = "contributions-by",
        graphql(description = "Author of additional contributions to the work")
    )]
    ContributionsBy,
    #[cfg_attr(feature = "backend", graphql(description = "Compiler of index"))]
    Indexer,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize, EnumString, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "title_case")]
pub enum WorkStatus {
    Unspecified,
    Cancelled,
    Forthcoming,
    #[cfg_attr(feature = "backend", db_rename = "postponed-indefinitely")]
    PostponedIndefinitely,
    Active,
    #[cfg_attr(feature = "backend", db_rename = "no-longer-our-product")]
    NoLongerOurProduct,
    #[cfg_attr(feature = "backend", db_rename = "out-of-stock-indefinitely")]
    OutOfStockIndefinitely,
    #[cfg_attr(feature = "backend", db_rename = "out-of-print")]
    OutOfPrint,
    #[default]
    Inactive,
    Unknown,
    Remaindered,
    #[cfg_attr(feature = "backend", db_rename = "withdrawn-from-sale")]
    WithdrawnFromSale,
    Recalled,
}

#[derive(Error, Debug, PartialEq, Eq)]
/// Represents anything that can go wrong in Thoth
///
/// This type is not intended to be exhaustively matched, and new variants may
/// be added in the future without a major version bump.
pub enum ThothError {
    #[error("{0} is not a valid {1} code")]
    InvalidSubjectCode(String, String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("{0}")]
    DatabaseConstraintError(&'static str),
    #[error("Internal error: {0}")]
    InternalError(String),
    #[error("Invalid credentials.")]
    Unauthorised,
    #[error("Failed to validate token.")]
    InvalidToken,
    #[error("No record was found for the given ID.")]
    EntityNotFound,
    #[error("Issue's Work and Series cannot have different Imprints.")]
    IssueImprintsError,
    #[error("{0} is not a valid metadata specification")]
    InvalidMetadataSpecification(String),
    #[error("Invalid UUID supplied.")]
    InvalidUuid,
    #[error("CSV Error: {0}")]
    CsvError(String),
    #[error("MARC Error: {0}")]
    MarcError(String),
    #[error("Could not generate {0}: {1}")]
    IncompleteMetadataRecord(String, String),
    #[error("{0} is not a validly formatted ORCID and will not be saved")]
    OrcidParseError(String),
    #[error("{0} is not a validly formatted DOI and will not be saved")]
    DoiParseError(String),
    #[error("{0} is not a validly formatted ISBN and will not be saved")]
    IsbnParseError(String),
    #[error("{0} is not a validly formatted ROR ID and will not be saved")]
    RorParseError(String),
    #[error("Cannot parse ORCID: no value provided")]
    OrcidEmptyError,
    #[error("Cannot parse DOI: no value provided")]
    DoiEmptyError,
    #[error("Cannot parse ISBN: no value provided")]
    IsbnEmptyError,
    #[error("Cannot parse ROR ID: no value provided")]
    RorEmptyError,
    #[error("Works of type Book Chapter cannot have ISBNs in their Publications.")]
    ChapterIsbnError,
    #[error(
        "Works of type Book Chapter cannot have Width, Height, Depth or Weight in their Publications."
    )]
    ChapterDimensionError,
    #[error("Each Publication must have exactly one canonical Location.")]
    CanonicalLocationError,
    #[error(
        "Canonical Locations for digital Publications must have both a Landing Page and a Full Text URL."
    )]
    LocationUrlError,
    #[error("When specifying Weight, both values (g and oz) must be supplied.")]
    WeightEmptyError,
    #[error("When specifying Width, both values (mm and in) must be supplied.")]
    WidthEmptyError,
    #[error("When specifying Height, both values (mm and in) must be supplied.")]
    HeightEmptyError,
    #[error("When specifying Depth, both values (mm and in) must be supplied.")]
    DepthEmptyError,
    #[error(
        "Width/Height/Depth/Weight are only applicable to physical (Paperback/Hardback) Publications."
    )]
    DimensionDigitalError,
    #[error(
        "Price values must be greater than zero. To indicate an unpriced Publication, omit all Prices."
    )]
    PriceZeroError,
    #[error("{0}")]
    RequestError(String),
    #[error("{0}")]
    GraphqlError(String),
    #[error("Withdrawn Date must be later than Publication Date.")]
    WithdrawnDateBeforePublicationDateError,
    #[error("Withdrawn Date can only be added to an Out of Print or Withdrawn From Sale Work.")]
    WithdrawnDateError,
    #[error("An Out of Print or Withdrawn From Sale Work must have a Withdrawn Date.")]
    NoWithdrawnDateError,
}

#[cfg(not(target_arch = "wasm32"))]
impl juniper::IntoFieldError for ThothError {
    fn into_field_error(self) -> juniper::FieldError {
        use juniper::graphql_value;
        match self {
            ThothError::InvalidSubjectCode { .. } => juniper::FieldError::new(
                self.to_string(),
                graphql_value!({
                    "type": "INVALID_SUBJECT_CODE"
                }),
            ),
            ThothError::Unauthorised => juniper::FieldError::new(
                "Unauthorized",
                graphql_value!({
                    "type": "NO_ACCESS"
                }),
            ),
            _ => juniper::FieldError::new(
                self.to_string(),
                graphql_value!({
                    "type": "INTERNAL_ERROR"
                }),
            ),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PublicationTypeValues {
    pub name: PublicationType,
}

impl From<yewtil::fetch::FetchError> for ThothError {
    fn from(error: yewtil::fetch::FetchError) -> Self {
        use serde_json::error::Result;
        use yewtil::fetch::FetchError;
        match error {
            FetchError::DeserializeError { error: _, content } => {
                let message: Result<GraqphqlErrorMessage> = serde_json::from_str(&content);
                match message {
                    Ok(m) => ThothError::GraphqlError(m.to_string()),
                    Err(_) => ThothError::RequestError(content),
                }
            }
            FetchError::CouldNotCreateFetchFuture => {
                ThothError::RequestError("Could not connect to the API.".to_string())
            }
            _ => ThothError::RequestError(error.to_string()),
        }
    }
}

#[derive(Debug, Deserialize)]
struct GraqphqlErrorMessage {
    errors: Vec<GraphqlError>,
}

impl fmt::Display for GraphqlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Display for GraqphqlErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for error in &self.errors {
            write!(f, "{error}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct GraphqlError {
    message: String,
}

#[cfg(not(target_arch = "wasm32"))]
impl From<csv::Error> for ThothError {
    fn from(e: csv::Error) -> Self {
        ThothError::CsvError(e.to_string())
    }
}

impl From<std::io::Error> for ThothError {
    fn from(error: std::io::Error) -> ThothError {
        ThothError::InternalError(error.to_string())
    }
}

impl From<&std::io::Error> for ThothError {
    fn from(error: &std::io::Error) -> ThothError {
        ThothError::InternalError(error.to_string())
    }
}

impl From<reqwest::Error> for ThothError {
    fn from(error: reqwest::Error) -> ThothError {
        ThothError::InternalError(error.to_string())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<reqwest_middleware::Error> for ThothError {
    fn from(error: reqwest_middleware::Error) -> ThothError {
        ThothError::InternalError(error.to_string())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<xml::writer::Error> for ThothError {
    fn from(error: xml::writer::Error) -> ThothError {
        ThothError::InternalError(error.to_string())
    }
}

impl From<uuid::Error> for ThothError {
    fn from(_: uuid::Error) -> ThothError {
        ThothError::InvalidUuid
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<marc::Error> for ThothError {
    fn from(e: marc::Error) -> Self {
        ThothError::MarcError(e.to_string())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<dialoguer::Error> for ThothError {
    fn from(e: dialoguer::Error) -> Self {
        ThothError::InternalError(e.to_string())
    }
}

impl Default for Timestamp {
    fn default() -> Timestamp {
        Timestamp(TimeZone::timestamp_opt(&Utc, 0, 0).unwrap())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Doi(String);

impl fmt::Display for Doi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0.replace(DOI_DOMAIN, ""))
    }
}

pub trait UrlIdentifier {
    fn domain(&self) -> &'static str;
}

impl UrlIdentifier for Doi {
    fn domain(&self) -> &'static str {
        DOI_DOMAIN
    }
}

impl Contribution {
    pub fn contribution_id(&self) -> Uuid {
        self.contribution_id
    }

    pub fn contributor_id(&self) -> Uuid {
        self.contributor_id
    }

    pub fn work_id(&self) -> Uuid {
        self.work_id
    }

    pub fn contribution_type(&self) -> &ContributionType {
        &self.contribution_type
    }

    pub fn main_contribution(&self) -> bool {
        self.main_contribution
    }

    pub fn biography(&self) -> Option<&String> {
        self.biography.as_ref()
    }

    pub fn created_at(&self) -> Timestamp {
        self.created_at.clone()
    }

    pub fn updated_at(&self) -> Timestamp {
        self.updated_at.clone()
    }

    pub fn first_name(&self) -> Option<&String> {
        self.first_name.as_ref()
    }

    pub fn last_name(&self) -> &String {
        &self.last_name
    }

    pub fn full_name(&self) -> &String {
        &self.full_name
    }

    pub fn contribution_ordinal(&self) -> &i32 {
        &self.contribution_ordinal
    }
}

pub struct AffiliationOrderBy {
    pub field: AffiliationField,
    pub direction: Direction,
}

pub enum AffiliationField {
    AffiliationId,
    ContributionId,
    InstitutionId,
    AffiliationOrdinal,
    Position,
    CreatedAt,
    UpdatedAt,
}

pub type FieldResult<T, S = DefaultScalarValue> = Result<T, FieldError<S>>;

#[derive(Debug, PartialEq, Clone, GraphQLScalarValue)]
#[allow(missing_docs)]
pub enum DefaultScalarValue {
    Int(i32),
    Float(f64),
    String(String),
    Boolean(bool),
}

impl ScalarValue for DefaultScalarValue {
    type Visitor = DefaultScalarValueVisitor;

    fn as_int(&self) -> Option<i32> {
        match *self {
            Self::Int(ref i) => Some(*i),
            _ => None,
        }
    }

    fn as_float(&self) -> Option<f64> {
        match *self {
            Self::Int(ref i) => Some(*i as f64),
            Self::Float(ref f) => Some(*f),
            _ => None,
        }
    }

    fn as_str(&self) -> Option<&str> {
        match *self {
            Self::String(ref s) => Some(s.as_str()),
            _ => None,
        }
    }

    fn as_string(&self) -> Option<String> {
        match *self {
            Self::String(ref s) => Some(s.clone()),
            _ => None,
        }
    }

    fn into_string(self) -> Option<String> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    fn as_boolean(&self) -> Option<bool> {
        match *self {
            Self::Boolean(ref b) => Some(*b),
            _ => None,
        }
    }

    fn into_another<S: ScalarValue>(self) -> S {
        match self {
            Self::Int(i) => S::from(i),
            Self::Float(f) => S::from(f),
            Self::String(s) => S::from(s),
            Self::Boolean(b) => S::from(b),
        }
    }
}

impl<'a> From<&'a str> for DefaultScalarValue {
    fn from(s: &'a str) -> Self {
        Self::String(s.into())
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct DefaultScalarValueVisitor;

impl<'de> de::Visitor<'de> for DefaultScalarValueVisitor {
    type Value = DefaultScalarValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid input value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<DefaultScalarValue, E> {
        Ok(DefaultScalarValue::Boolean(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<DefaultScalarValue, E>
    where
        E: de::Error,
    {
        if value >= i64::from(i32::min_value()) && value <= i64::from(i32::max_value()) {
            Ok(DefaultScalarValue::Int(value as i32))
        } else {
            // Browser's JSON.stringify serialize all numbers having no
            // fractional part as integers (no decimal point), so we
            // must parse large integers as floating point otherwise
            // we would error on transferring large floating point
            // numbers.
            Ok(DefaultScalarValue::Float(value as f64))
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<DefaultScalarValue, E>
    where
        E: de::Error,
    {
        if value <= i32::max_value() as u64 {
            self.visit_i64(value as i64)
        } else {
            // Browser's JSON.stringify serialize all numbers having no
            // fractional part as integers (no decimal point), so we
            // must parse large integers as floating point otherwise
            // we would error on transferring large floating point
            // numbers.
            Ok(DefaultScalarValue::Float(value as f64))
        }
    }

    fn visit_f64<E>(self, value: f64) -> Result<DefaultScalarValue, E> {
        Ok(DefaultScalarValue::Float(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<DefaultScalarValue, E>
    where
        E: de::Error,
    {
        self.visit_string(value.into())
    }

    fn visit_string<E>(self, value: String) -> Result<DefaultScalarValue, E> {
        Ok(DefaultScalarValue::String(value))
    }
}
#[derive(Debug, PartialEq)]
pub struct FieldError<S = DefaultScalarValue> {
    message: String,
    extensions: Value<S>,
}

impl<S> FieldError<S> {
    /// Construct a new error with additional data
    ///
    /// You can use the `graphql_value!` macro to construct an error:
    ///
    /// ```rust
    /// use juniper::FieldError;
    /// # use juniper::DefaultScalarValue;
    /// use juniper::graphql_value;
    ///
    /// # fn sample() {
    /// # let _: FieldError<DefaultScalarValue> =
    /// FieldError::new(
    ///     "Could not open connection to the database",
    ///     graphql_value!({ "internal_error": "Connection refused" })
    /// );
    /// # }
    /// # fn main() { }
    /// ```
    ///
    /// The `extensions` parameter will be added to the `"extensions"` field of the error
    /// object in the JSON response:
    ///
    /// ```json
    /// {
    ///   "errors": [
    ///     "message": "Could not open connection to the database",
    ///     "locations": [{"line": 2, "column": 4}],
    ///     "extensions": {
    ///       "internal_error": "Connection refused"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// If the argument is `Value::null()`, no extra data will be included.
    pub fn new<T: Display>(e: T, extensions: Value<S>) -> FieldError<S> {
        FieldError {
            message: format!("{}", e),
            extensions,
        }
    }

    #[doc(hidden)]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[doc(hidden)]
    pub fn extensions(&self) -> &Value<S> {
        &self.extensions
    }

    /// Maps the [`ScalarValue`] type of this [`FieldError`] into the specified one.
    pub fn map_scalar_value<Into>(self) -> FieldError<Into>
    where
        S: ScalarValue,
        Into: ScalarValue,
    {
        FieldError {
            message: self.message,
            extensions: self.extensions.map_scalar_value(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Affiliation {
    pub affiliation_id: Uuid,
    pub contribution_id: Uuid,
    pub institution_id: Uuid,
    pub affiliation_ordinal: i32,
    pub position: Option<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

pub trait ListString {
    const BULLET_SEPARATOR: &'static str = " • ";
    const COMMA_SEPARATOR: &'static str = ", ";

    fn separated_list_item_comma(&self) -> Html {
        self.separated_list_item(false, Self::COMMA_SEPARATOR)
    }

    fn separated_list_item(&self, is_small: bool, separator: &str) -> Html;
}

impl ListString for Contribution {
    fn separated_list_item(&self, is_small: bool, separator: &str) -> Html {
        // Only include contributions marked as "Main" in summary list
        if self.main_contribution {
            if is_small {
                html! {
                    <small class="contributor">
                        {&self.full_name}
                        <span>{ separator }</span>
                    </small>
                }
            } else {
                html! {
                    <span class="contributor">
                        {&self.full_name}
                        <span>{ separator }</span>
                    </span>
                }
            }
        } else {
            html! {}
        }
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0.format("%F %T"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WorkTypeValues {
    pub name: WorkType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WorkStatusValues {
    pub name: WorkStatus,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WorkStatusDefinition {
    pub enum_values: Vec<WorkStatusValues>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WorkTypeDefinition {
    pub enum_values: Vec<WorkTypeValues>,
}
