use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4002410025: FileType = FileType {
    file_format: &FileFormat {
        id: 4_002_410_025,
        source_type: SourceType::Iana,
        name: "tamp-sequence-adjust-confirm",
        extensions: &[],
        media_types: &["application/tamp-sequence-adjust-confirm"],
        signatures: &[],
        related_formats: &[],
    },
};
