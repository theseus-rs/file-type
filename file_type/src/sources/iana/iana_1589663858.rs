use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1589663858: FileFormat = FileFormat {
    id: 1_589_663_858,
    source_type: SourceType::Iana,
    name: "vnd.wordlift",
    extensions: &[],
    media_types: &["application/vnd.wordlift"],
    internal_signatures: &[],
    related_formats: &[],
};
