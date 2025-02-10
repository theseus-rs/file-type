use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2401825161: FileType = FileType {
    file_format: &FileFormat {
        id: 2_401_825_161,
        source_type: SourceType::Iana,
        name: "avif",
        extensions: &[],
        media_types: &["image/avif"],
        signatures: &[],
        related_formats: &[],
    },
};
