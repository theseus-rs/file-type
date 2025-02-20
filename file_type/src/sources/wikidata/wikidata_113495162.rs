use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113495162: FileType = FileType {
    file_format: &FileFormat {
        id: 113_495_162,
        source_type: SourceType::Wikidata,
        name: "Calc602 Project File 1.0",
        extensions: &["pc6"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
