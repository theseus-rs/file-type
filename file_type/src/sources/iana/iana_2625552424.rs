use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2625552424: FileType = FileType {
    file_format: &FileFormat {
        id: 2_625_552_424,
        source_type: SourceType::Iana,
        name: "vnd.wolfram.player",
        extensions: &[],
        media_types: &["application/vnd.wolfram.player"],
        signatures: &[],
        related_formats: &[],
    },
};
