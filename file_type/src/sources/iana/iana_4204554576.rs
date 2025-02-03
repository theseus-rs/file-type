use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4204554576: FileFormat = FileFormat {
    id: 4_204_554_576,
    source_type: SourceType::Iana,
    name: "vnd.ms-project",
    extensions: &[],
    media_types: &["application/vnd.ms-project"],
    internal_signatures: &[],
    related_formats: &[],
};
