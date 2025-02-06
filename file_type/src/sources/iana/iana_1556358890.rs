use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1556358890: FileFormat = FileFormat {
    id: 1_556_358_890,
    source_type: SourceType::Iana,
    name: "jxsc",
    extensions: &[],
    media_types: &["image/jxsc"],
    signatures: &[],
    related_formats: &[],
};
