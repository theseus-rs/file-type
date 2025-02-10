use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3834962646: FileType = FileType {
    file_format: &FileFormat {
        id: 3_834_962_646,
        source_type: SourceType::Iana,
        name: "ssml+xml",
        extensions: &[],
        media_types: &["application/ssml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
