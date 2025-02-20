use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1447934773: FileType = FileType {
    file_format: &FileFormat {
        id: 1_447_934_773,
        source_type: SourceType::Iana,
        name: "vnd.iptvforum.ttsavc",
        extensions: &[],
        media_types: &["video/vnd.iptvforum.ttsavc"],
        signatures: &[],
        related_formats: &[],
    },
};
