use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
