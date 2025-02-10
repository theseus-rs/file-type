use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27861482: FileType = FileType {
    file_format: &FileFormat {
        id: 27_861_482,
        source_type: SourceType::Wikidata,
        name: "Renoise Song, version 10",
        extensions: &["xrns"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
