use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113472408: FileType = FileType {
    file_format: &FileFormat {
        id: 113_472_408,
        source_type: SourceType::Wikidata,
        name: "Glyphs Character Data",
        extensions: &["glyphs"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
