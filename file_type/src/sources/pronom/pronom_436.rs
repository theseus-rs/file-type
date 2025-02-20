use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_436: FileType = FileType {
    file_format: &FileFormat {
        id: 436,
        source_type: SourceType::Pronom,
        name: "IBM DisplayWrite Revisable Form Text File",
        extensions: &["rft"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
