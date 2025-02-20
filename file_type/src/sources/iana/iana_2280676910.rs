use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2280676910: FileType = FileType {
    file_format: &FileFormat {
        id: 2_280_676_910,
        source_type: SourceType::Iana,
        name: "vnd.publishare-delta-tree",
        extensions: &[],
        media_types: &["application/vnd.publishare-delta-tree"],
        signatures: &[],
        related_formats: &[],
    },
};
