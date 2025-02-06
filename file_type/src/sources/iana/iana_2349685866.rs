use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2349685866: FileFormat = FileFormat {
    id: 2_349_685_866,
    source_type: SourceType::Iana,
    name: "vnd.groove-account",
    extensions: &[],
    media_types: &["application/vnd.groove-account"],
    signatures: &[],
    related_formats: &[],
};
