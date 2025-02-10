use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3220007017: FileType = FileType {
    file_format: &FileFormat {
        id: 3_220_007_017,
        source_type: SourceType::Iana,
        name: "pointer",
        extensions: &[],
        media_types: &["video/pointer"],
        signatures: &[],
        related_formats: &[],
    },
};
