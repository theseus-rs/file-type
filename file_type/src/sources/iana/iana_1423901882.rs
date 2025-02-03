use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1423901882: FileFormat = FileFormat {
    id: 1_423_901_882,
    source_type: SourceType::Iana,
    name: "vnd.dolby.mps",
    extensions: &[],
    media_types: &["audio/vnd.dolby.mps"],
    internal_signatures: &[],
    related_formats: &[],
};
