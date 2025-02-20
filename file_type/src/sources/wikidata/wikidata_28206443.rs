use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206443: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_443,
        source_type: SourceType::Wikidata,
        name: "Kt Interchange File Format",
        extensions: &["kif", "kiff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
