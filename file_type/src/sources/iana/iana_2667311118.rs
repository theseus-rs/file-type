use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2667311118: FileFormat = FileFormat {
    id: 2_667_311_118,
    source_type: SourceType::Iana,
    name: "EVRC",
    extensions: &[],
    media_types: &["audio/EVRC"],
    internal_signatures: &[],
    related_formats: &[],
};
