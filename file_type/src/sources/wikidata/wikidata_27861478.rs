use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27861478: FileType = FileType {
    file_format: &FileFormat {
        id: 27_861_478,
        source_type: SourceType::Wikidata,
        name: "Renoise Song, version 4",
        extensions: &["xrns"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
