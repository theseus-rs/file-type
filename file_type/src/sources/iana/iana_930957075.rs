use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_930957075: FileFormat = FileFormat {
    id: 930_957_075,
    source_type: SourceType::Iana,
    name: "vnd.mynfc",
    extensions: &[],
    media_types: &["application/vnd.mynfc"],
    signatures: &[],
    related_formats: &[],
};
