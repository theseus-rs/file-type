use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_324707210: FileType = FileType {
    file_format: &FileFormat {
        id: 324_707_210,
        source_type: SourceType::Iana,
        name: "vnd.ms-powerpoint.slideshow.macroEnabled.12",
        extensions: &[],
        media_types: &["application/vnd.ms-powerpoint.slideshow.macroEnabled.12"],
        signatures: &[],
        related_formats: &[],
    },
};
