use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2628327227: FileType = FileType {
    file_format: &FileFormat {
        id: 2_628_327_227,
        source_type: SourceType::Iana,
        name: "senml+xml",
        extensions: &[],
        media_types: &["application/senml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
