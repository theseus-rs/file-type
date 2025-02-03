use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1246830443: FileFormat = FileFormat {
    id: 1_246_830_443,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
