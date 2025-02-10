use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1632936163: FileType = FileType {
    file_format: &FileFormat {
        id: 1_632_936_163,
        source_type: SourceType::Iana,
        name: "tamp-apex-update",
        extensions: &[],
        media_types: &["application/tamp-apex-update"],
        signatures: &[],
        related_formats: &[],
    },
};
