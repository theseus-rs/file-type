use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2121: FileType = FileType {
    file_format: &FileFormat {
        id: 2_121,
        source_type: SourceType::Pronom,
        name: "Microsoft Shell Scrap Object File",
        extensions: &["shs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
