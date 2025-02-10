use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1314571476: FileType = FileType {
    file_format: &FileFormat {
        id: 1_314_571_476,
        source_type: SourceType::Iana,
        name: "vnd.nokia.mp4vr",
        extensions: &[],
        media_types: &["video/vnd.nokia.mp4vr"],
        signatures: &[],
        related_formats: &[],
    },
};
