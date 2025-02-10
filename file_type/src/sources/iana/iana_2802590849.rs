use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2802590849: FileType = FileType {
    file_format: &FileFormat {
        id: 2_802_590_849,
        source_type: SourceType::Iana,
        name: "mpeg4-generic",
        extensions: &[],
        media_types: &["video/mpeg4-generic"],
        signatures: &[],
        related_formats: &[],
    },
};
