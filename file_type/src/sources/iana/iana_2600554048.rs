use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2600554048: FileFormat = FileFormat {
    id: 2_600_554_048,
    source_type: SourceType::Iana,
    name: "mikey",
    extensions: &[],
    media_types: &["application/mikey"],
    signatures: &[],
    related_formats: &[],
};
