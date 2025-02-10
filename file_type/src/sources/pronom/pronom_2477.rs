use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2477: FileType = FileType {
    file_format: &FileFormat {
        id: 2_477,
        source_type: SourceType::Pronom,
        name: "Bayesian Interchange Format File",
        extensions: &["bif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
