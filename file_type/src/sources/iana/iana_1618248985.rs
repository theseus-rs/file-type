use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1618248985: FileType = FileType {
    file_format: &FileFormat {
        id: 1_618_248_985,
        source_type: SourceType::Iana,
        name: "tamp-update",
        extensions: &[],
        media_types: &["application/tamp-update"],
        signatures: &[],
        related_formats: &[],
    },
};
