use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2148: FileType = FileType {
    file_format: &FileFormat {
        id: 2_148,
        source_type: SourceType::Pronom,
        name: "Avery DesignPro Document",
        extensions: &["zdp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
