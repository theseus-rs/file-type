use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2166160279: FileType = FileType {
    file_format: &FileFormat {
        id: 2_166_160_279,
        source_type: SourceType::Iana,
        name: "vnd.cybank",
        extensions: &[],
        media_types: &["application/vnd.cybank"],
        signatures: &[],
        related_formats: &[],
    },
};
