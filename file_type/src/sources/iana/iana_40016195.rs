use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_40016195: FileFormat = FileFormat {
    id: 40_016_195,
    source_type: SourceType::Iana,
    name: "activity+json",
    extensions: &[],
    media_types: &["application/activity+json"],
    internal_signatures: &[],
    related_formats: &[],
};
