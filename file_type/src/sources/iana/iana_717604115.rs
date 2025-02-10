use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_717604115: FileType = FileType {
    file_format: &FileFormat {
        id: 717_604_115,
        source_type: SourceType::Iana,
        name: "vnd.oai.workflows+json",
        extensions: &[],
        media_types: &["application/vnd.oai.workflows+json"],
        signatures: &[],
        related_formats: &[],
    },
};
