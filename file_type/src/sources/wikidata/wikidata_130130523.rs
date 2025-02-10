use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130130523: FileType = FileType {
    file_format: &FileFormat {
        id: 130_130_523,
        source_type: SourceType::Wikidata,
        name: "Kuin source code file",
        extensions: &["kn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
