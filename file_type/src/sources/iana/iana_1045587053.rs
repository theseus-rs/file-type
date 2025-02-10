use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1045587053: FileType = FileType {
    file_format: &FileFormat {
        id: 1_045_587_053,
        source_type: SourceType::Iana,
        name: "BV16",
        extensions: &[],
        media_types: &["audio/BV16"],
        signatures: &[],
        related_formats: &[],
    },
};
