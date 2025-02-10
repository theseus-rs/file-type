use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_816441770: FileType = FileType {
    file_format: &FileFormat {
        id: 816_441_770,
        source_type: SourceType::Iana,
        name: "rtp-enc-aescm128",
        extensions: &[],
        media_types: &["video/rtp-enc-aescm128"],
        signatures: &[],
        related_formats: &[],
    },
};
