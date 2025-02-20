use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967217: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_217,
        source_type: SourceType::Wikidata,
        name: "Scream Tracker Music Interface Kit module",
        extensions: &["stx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
