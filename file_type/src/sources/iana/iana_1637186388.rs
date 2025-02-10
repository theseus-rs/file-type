use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1637186388: FileType = FileType {
    file_format: &FileFormat {
        id: 1_637_186_388,
        source_type: SourceType::Iana,
        name: "vnd.music-niff",
        extensions: &[],
        media_types: &["application/vnd.music-niff"],
        signatures: &[],
        related_formats: &[],
    },
};
