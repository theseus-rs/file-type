use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2023: FileType = FileType {
    file_format: &FileFormat {
        id: 2_023,
        source_type: SourceType::Pronom,
        name: "Zoner Callisto Metafile",
        extensions: &["zmf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
