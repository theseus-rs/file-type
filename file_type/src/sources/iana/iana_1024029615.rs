use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1024029615: FileFormat = FileFormat {
    id: 1_024_029_615,
    source_type: SourceType::Iana,
    name: "vnd.commerce-battelle",
    extensions: &[],
    media_types: &["application/vnd.commerce-battelle"],
    signatures: &[],
    related_formats: &[],
};
