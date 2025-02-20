use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_878383878: FileType = FileType {
    file_format: &FileFormat {
        id: 878_383_878,
        source_type: SourceType::Iana,
        name: "vnd.motorola.videop",
        extensions: &[],
        media_types: &["video/vnd.motorola.videop"],
        signatures: &[],
        related_formats: &[],
    },
};
