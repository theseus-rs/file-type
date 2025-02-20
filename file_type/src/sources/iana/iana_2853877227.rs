use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
