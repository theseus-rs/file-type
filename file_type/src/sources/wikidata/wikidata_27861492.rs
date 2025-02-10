use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27861492: FileType = FileType {
    file_format: &FileFormat {
        id: 27_861_492,
        source_type: SourceType::Wikidata,
        name: "Renoise Song, version 37",
        extensions: &["xrns"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
