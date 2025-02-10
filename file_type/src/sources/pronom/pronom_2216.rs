use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2216: FileType = FileType {
    file_format: &FileFormat {
        id: 2_216,
        source_type: SourceType::Pronom,
        name: "FARO WorkSpace File",
        extensions: &["fws"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
