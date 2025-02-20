use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4029535853: FileType = FileType {
    file_format: &FileFormat {
        id: 4_029_535_853,
        source_type: SourceType::Iana,
        name: "webp",
        extensions: &[],
        media_types: &["image/webp"],
        signatures: &[],
        related_formats: &[],
    },
};
