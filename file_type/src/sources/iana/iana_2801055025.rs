use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2801055025: FileType = FileType {
    file_format: &FileFormat {
        id: 2_801_055_025,
        source_type: SourceType::Iana,
        name: "H261",
        extensions: &[],
        media_types: &["video/H261"],
        signatures: &[],
        related_formats: &[],
    },
};
