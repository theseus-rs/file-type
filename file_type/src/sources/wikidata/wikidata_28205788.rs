use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205788: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_788,
        source_type: SourceType::Wikidata,
        name: "Compact Picture Format",
        extensions: &["cpt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
