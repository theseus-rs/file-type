use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1431669032: FileType = FileType {
    file_format: &FileFormat {
        id: 1_431_669_032,
        source_type: SourceType::Iana,
        name: "vnd.hns.video",
        extensions: &[],
        media_types: &["video/vnd.hns.video"],
        signatures: &[],
        related_formats: &[],
    },
};
