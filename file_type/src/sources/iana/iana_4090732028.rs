use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4090732028: FileType = FileType {
    file_format: &FileFormat {
        id: 4_090_732_028,
        source_type: SourceType::Iana,
        name: "EVS",
        extensions: &[],
        media_types: &["audio/EVS"],
        signatures: &[],
        related_formats: &[],
    },
};
