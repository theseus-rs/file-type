use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2954147763: FileFormat = FileFormat {
    id: 2_954_147_763,
    source_type: SourceType::Iana,
    name: "vnd.uplanet.bearer-choice-wbxml",
    extensions: &[],
    media_types: &["application/vnd.uplanet.bearer-choice-wbxml"],
    signatures: &[],
    related_formats: &[],
};
