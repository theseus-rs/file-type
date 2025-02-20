use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1958789691: FileType = FileType {
    file_format: &FileFormat {
        id: 1_958_789_691,
        source_type: SourceType::Iana,
        name: "vnd.motorola.video",
        extensions: &[],
        media_types: &["video/vnd.motorola.video"],
        signatures: &[],
        related_formats: &[],
    },
};
