use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_132065091: FileType = FileType {
    file_format: &FileFormat {
        id: 132_065_091,
        source_type: SourceType::Iana,
        name: "iso.segment",
        extensions: &[],
        media_types: &["video/iso.segment"],
        signatures: &[],
        related_formats: &[],
    },
};
