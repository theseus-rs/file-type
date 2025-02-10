use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1968309032: FileType = FileType {
    file_format: &FileFormat {
        id: 1_968_309_032,
        source_type: SourceType::Iana,
        name: "vnd.recordare.musicxml",
        extensions: &[],
        media_types: &["application/vnd.recordare.musicxml"],
        signatures: &[],
        related_formats: &[],
    },
};
