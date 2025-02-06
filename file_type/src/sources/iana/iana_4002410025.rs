use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4002410025: FileFormat = FileFormat {
    id: 4_002_410_025,
    source_type: SourceType::Iana,
    name: "tamp-sequence-adjust-confirm",
    extensions: &[],
    media_types: &["application/tamp-sequence-adjust-confirm"],
    signatures: &[],
    related_formats: &[],
};
