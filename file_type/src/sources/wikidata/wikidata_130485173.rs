use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130485173: FileType = FileType {
    file_format: &FileFormat {
        id: 130_485_173,
        source_type: SourceType::Wikidata,
        name: "Portugol file format",
        extensions: &["alg", "portugol"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
