use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_539: FileType = FileType {
    file_format: &FileFormat {
        id: 539,
        source_type: SourceType::Pronom,
        name: "Vista Pro Graphics",
        extensions: &["dem"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
