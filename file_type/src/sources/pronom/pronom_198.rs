use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_198: FileType = FileType {
    file_format: &FileFormat {
        id: 198,
        source_type: SourceType::Pronom,
        name: "Active Server Page",
        extensions: &["asp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
