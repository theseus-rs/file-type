use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_670748266: FileType = FileType {
    file_format: &FileFormat {
        id: 670_748_266,
        source_type: SourceType::Iana,
        name: "ttf",
        extensions: &[],
        media_types: &["font/ttf"],
        signatures: &[],
        related_formats: &[],
    },
};
