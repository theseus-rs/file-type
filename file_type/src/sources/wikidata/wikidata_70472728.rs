use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_70472728: FileType = FileType {
    file_format: &FileFormat {
        id: 70_472_728,
        source_type: SourceType::Wikidata,
        name: "ReproZip Pack Format",
        extensions: &["rpz"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
