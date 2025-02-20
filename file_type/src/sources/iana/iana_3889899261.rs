use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3889899261: FileType = FileType {
    file_format: &FileFormat {
        id: 3_889_899_261,
        source_type: SourceType::Iana,
        name: "3gppHal+json",
        extensions: &[],
        media_types: &["application/3gppHal+json"],
        signatures: &[],
        related_formats: &[],
    },
};
