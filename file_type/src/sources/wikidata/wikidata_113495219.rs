use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113495219: FileType = FileType {
    file_format: &FileFormat {
        id: 113_495_219,
        source_type: SourceType::Wikidata,
        name: "CATIA Model File 3",
        extensions: &["model"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
