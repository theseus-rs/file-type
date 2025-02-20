use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125173042: FileType = FileType {
    file_format: &FileFormat {
        id: 125_173_042,
        source_type: SourceType::Wikidata,
        name: "Tomboy note",
        extensions: &["note"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
