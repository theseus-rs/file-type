use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_588578662: FileFormat = FileFormat {
    id: 588_578_662,
    source_type: SourceType::Iana,
    name: "VP9",
    extensions: &[],
    media_types: &["video/VP9"],
    internal_signatures: &[],
    related_formats: &[],
};
