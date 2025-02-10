use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_272: FileType = FileType {
    file_format: &FileFormat {
        id: 272,
        source_type: SourceType::Pronom,
        name: "Pagemaker TableEditor Graphics",
        extensions: &["tbl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
