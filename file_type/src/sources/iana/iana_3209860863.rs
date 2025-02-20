use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3209860863: FileType = FileType {
    file_format: &FileFormat {
        id: 3_209_860_863,
        source_type: SourceType::Iana,
        name: "vnd.nokia.videovoip",
        extensions: &[],
        media_types: &["video/vnd.nokia.videovoip"],
        signatures: &[],
        related_formats: &[],
    },
};
