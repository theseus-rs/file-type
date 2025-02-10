use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4204554576: FileType = FileType {
    file_format: &FileFormat {
        id: 4_204_554_576,
        source_type: SourceType::Iana,
        name: "vnd.ms-project",
        extensions: &[],
        media_types: &["application/vnd.ms-project"],
        signatures: &[],
        related_formats: &[],
    },
};
