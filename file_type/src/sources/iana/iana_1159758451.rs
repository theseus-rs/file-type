use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1159758451: FileFormat = FileFormat {
    id: 1_159_758_451,
    source_type: SourceType::Iana,
    name: "3gpp-tt",
    extensions: &[],
    media_types: &["video/3gpp-tt"],
    internal_signatures: &[],
    related_formats: &[],
};
