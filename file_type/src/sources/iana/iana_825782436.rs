use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
