use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27861489: FileType = FileType {
    file_format: &FileFormat {
        id: 27_861_489,
        source_type: SourceType::Wikidata,
        name: "Renoise Song, version 22",
        extensions: &["xrns"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
