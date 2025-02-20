use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2764975073: FileType = FileType {
    file_format: &FileFormat {
        id: 2_764_975_073,
        source_type: SourceType::Iana,
        name: "vnd.recordare.musicxml+xml",
        extensions: &[],
        media_types: &["application/vnd.recordare.musicxml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
