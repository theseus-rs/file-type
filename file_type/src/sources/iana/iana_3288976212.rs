use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3288976212: FileFormat = FileFormat {
    id: 3_288_976_212,
    source_type: SourceType::Iana,
    name: "vnd.pmi.widget",
    extensions: &[],
    media_types: &["application/vnd.pmi.widget"],
    signatures: &[],
    related_formats: &[],
};
