use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2853877227: FileType = FileType {
    file_format: &FileFormat {
        id: 2_853_877_227,
        source_type: SourceType::Iana,
        name: "jphc",
        extensions: &[],
        media_types: &["image/jphc"],
        signatures: &[],
        related_formats: &[],
    },
};
