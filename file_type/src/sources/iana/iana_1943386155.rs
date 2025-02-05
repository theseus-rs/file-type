use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1943386155: FileFormat = FileFormat {
    id: 1_943_386_155,
    source_type: SourceType::Iana,
    name: "vnd.tmobile-livetv",
    extensions: &[],
    media_types: &["application/vnd.tmobile-livetv"],
    signatures: &[],
    related_formats: &[],
};
