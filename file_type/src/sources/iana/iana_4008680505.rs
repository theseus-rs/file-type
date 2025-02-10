use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4008680505: FileType = FileType {
    file_format: &FileFormat {
        id: 4_008_680_505,
        source_type: SourceType::Iana,
        name: "vnd.crick.clicker",
        extensions: &[],
        media_types: &["application/vnd.crick.clicker"],
        signatures: &[],
        related_formats: &[],
    },
};
