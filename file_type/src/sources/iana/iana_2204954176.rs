use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2204954176: FileFormat = FileFormat {
    id: 2_204_954_176,
    source_type: SourceType::Iana,
    name: "vnd.1ob",
    extensions: &[],
    media_types: &["application/vnd.1ob"],
    signatures: &[],
    related_formats: &[],
};
