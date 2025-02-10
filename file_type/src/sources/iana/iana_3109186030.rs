use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3109186030: FileType = FileType {
    file_format: &FileFormat {
        id: 3_109_186_030,
        source_type: SourceType::Iana,
        name: "BV32",
        extensions: &[],
        media_types: &["audio/BV32"],
        signatures: &[],
        related_formats: &[],
    },
};
