use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2375393172: FileType = FileType {
    file_format: &FileFormat {
        id: 2_375_393_172,
        source_type: SourceType::Iana,
        name: "vnd.presonus.multitrack",
        extensions: &[],
        media_types: &["audio/vnd.presonus.multitrack"],
        signatures: &[],
        related_formats: &[],
    },
};
