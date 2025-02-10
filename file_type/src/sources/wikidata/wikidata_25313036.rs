use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_25313036: FileType = FileType {
    file_format: &FileFormat {
        id: 25_313_036,
        source_type: SourceType::Wikidata,
        name: "Extensible Data Notation",
        extensions: &["edn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
