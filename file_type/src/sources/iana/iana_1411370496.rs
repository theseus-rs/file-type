use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1411370496: FileType = FileType {
    file_format: &FileFormat {
        id: 1_411_370_496,
        source_type: SourceType::Iana,
        name: "SMV",
        extensions: &[],
        media_types: &["audio/SMV"],
        signatures: &[],
        related_formats: &[],
    },
};
