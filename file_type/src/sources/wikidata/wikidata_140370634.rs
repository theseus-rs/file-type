use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_140370634: FileType = FileType {
    file_format: &FileFormat {
        id: 140_370_634,
        source_type: SourceType::Wikidata,
        name: "eMule Resource File",
        extensions: &["met"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
