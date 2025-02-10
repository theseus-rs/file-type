use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2923392260: FileType = FileType {
    file_format: &FileFormat {
        id: 2_923_392_260,
        source_type: SourceType::Iana,
        name: "vnd.hyper+json",
        extensions: &[],
        media_types: &["application/vnd.hyper+json"],
        signatures: &[],
        related_formats: &[],
    },
};
