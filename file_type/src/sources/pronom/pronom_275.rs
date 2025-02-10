use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_275: FileType = FileType {
    file_format: &FileFormat {
        id: 275,
        source_type: SourceType::Pronom,
        name: "CCITT G.711 Audio",
        extensions: &["ulaw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
