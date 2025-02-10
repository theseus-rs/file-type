use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2885619378: FileType = FileType {
    file_format: &FileFormat {
        id: 2_885_619_378,
        source_type: SourceType::Iana,
        name: "vnd.youtube.yt (OBSOLETED in favor of video/vnd.youtube.yt)",
        extensions: &[],
        media_types: &["application/vnd.youtube.yt"],
        signatures: &[],
        related_formats: &[],
    },
};
