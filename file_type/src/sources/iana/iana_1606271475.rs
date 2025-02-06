use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1606271475: FileFormat = FileFormat {
    id: 1_606_271_475,
    source_type: SourceType::Iana,
    name: "vnd.japannet-registration",
    extensions: &[],
    media_types: &["application/vnd.japannet-registration"],
    signatures: &[],
    related_formats: &[],
};
