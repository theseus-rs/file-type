use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2923392260: FileFormat = FileFormat {
    id: 2_923_392_260,
    source_type: SourceType::Iana,
    name: "vnd.hyper+json",
    extensions: &[],
    media_types: &["application/vnd.hyper+json"],
    internal_signatures: &[],
    related_formats: &[],
};
