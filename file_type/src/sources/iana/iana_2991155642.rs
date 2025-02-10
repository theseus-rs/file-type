use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2991155642: FileType = FileType {
    file_format: &FileFormat {
        id: 2_991_155_642,
        source_type: SourceType::Iana,
        name: "vnd.ipunplugged.rcprofile",
        extensions: &[],
        media_types: &["application/vnd.ipunplugged.rcprofile"],
        signatures: &[],
        related_formats: &[],
    },
};
