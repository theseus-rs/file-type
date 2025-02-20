use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_489: FileType = FileType {
    file_format: &FileFormat {
        id: 489,
        source_type: SourceType::Pronom,
        name: "Hewlett Packard AdvanceWrite Text File",
        extensions: &["aw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
