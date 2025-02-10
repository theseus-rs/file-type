use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2753039250: FileType = FileType {
    file_format: &FileFormat {
        id: 2_753_039_250,
        source_type: SourceType::Iana,
        name: "vnd.youtube.yt",
        extensions: &[],
        media_types: &["video/vnd.youtube.yt"],
        signatures: &[],
        related_formats: &[],
    },
};
