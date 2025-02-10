use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_825782436: FileType = FileType {
    file_format: &FileFormat {
        id: 825_782_436,
        source_type: SourceType::Iana,
        name: "wasm",
        extensions: &[],
        media_types: &["application/wasm"],
        signatures: &[],
        related_formats: &[],
    },
};
