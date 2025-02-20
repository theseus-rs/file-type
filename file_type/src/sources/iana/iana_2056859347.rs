use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2056859347: FileType = FileType {
    file_format: &FileFormat {
        id: 2_056_859_347,
        source_type: SourceType::Iana,
        name: "BT656",
        extensions: &[],
        media_types: &["video/BT656"],
        signatures: &[],
        related_formats: &[],
    },
};
