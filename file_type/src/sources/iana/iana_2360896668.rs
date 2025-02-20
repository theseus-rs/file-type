use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2360896668: FileType = FileType {
    file_format: &FileFormat {
        id: 2_360_896_668,
        source_type: SourceType::Iana,
        name: "vnd.iptvforum.ttsmpeg2",
        extensions: &[],
        media_types: &["video/vnd.iptvforum.ttsmpeg2"],
        signatures: &[],
        related_formats: &[],
    },
};
