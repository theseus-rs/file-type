use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_253: FileType = FileType {
    file_format: &FileFormat {
        id: 253,
        source_type: SourceType::Pronom,
        name: "Instalit Script",
        extensions: &["pvd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
