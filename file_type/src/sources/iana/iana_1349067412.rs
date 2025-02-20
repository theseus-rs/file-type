use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1349067412: FileType = FileType {
    file_format: &FileFormat {
        id: 1_349_067_412,
        source_type: SourceType::Iana,
        name: "G729E",
        extensions: &[],
        media_types: &["audio/G729E"],
        signatures: &[],
        related_formats: &[],
    },
};
