use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_825782436: FileType = FileType {
    file_format: &FileFormat {
        id: 825_782_436,
        source_type: SourceType::Httpd,
        name: "wasm",
        extensions: &["wasm"],
        media_types: &["application/wasm"],
        signatures: &[],
        related_formats: &[],
    },
};
