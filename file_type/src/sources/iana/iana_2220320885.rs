use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2220320885: FileType = FileType {
    file_format: &FileFormat {
        id: 2_220_320_885,
        source_type: SourceType::Iana,
        name: "t140",
        extensions: &[],
        media_types: &["text/t140"],
        signatures: &[],
        related_formats: &[],
    },
};
