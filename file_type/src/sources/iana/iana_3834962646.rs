use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3834962646: FileFormat = FileFormat {
    id: 3_834_962_646,
    source_type: SourceType::Iana,
    name: "ssml+xml",
    extensions: &[],
    media_types: &["application/ssml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
