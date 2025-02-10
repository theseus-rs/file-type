use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1765471557: FileType = FileType {
    file_format: &FileFormat {
        id: 1_765_471_557,
        source_type: SourceType::Iana,
        name: "aptx",
        extensions: &[],
        media_types: &["audio/aptx"],
        signatures: &[],
        related_formats: &[],
    },
};
