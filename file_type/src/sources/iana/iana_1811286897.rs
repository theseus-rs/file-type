use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1811286897: FileType = FileType {
    file_format: &FileFormat {
        id: 1_811_286_897,
        source_type: SourceType::Iana,
        name: "media_control+xml",
        extensions: &[],
        media_types: &["application/media_control+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
