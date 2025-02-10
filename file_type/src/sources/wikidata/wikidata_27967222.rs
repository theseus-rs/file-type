use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967222: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_222,
        source_type: SourceType::Wikidata,
        name: "Soundtrakker 128 module",
        extensions: &["128"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
