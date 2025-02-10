use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_886513491: FileType = FileType {
    file_format: &FileFormat {
        id: 886_513_491,
        source_type: SourceType::Iana,
        name: "emma+xml",
        extensions: &[],
        media_types: &["application/emma+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
