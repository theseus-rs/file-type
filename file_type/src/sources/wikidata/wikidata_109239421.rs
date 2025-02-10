use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109239421: FileType = FileType {
    file_format: &FileFormat {
        id: 109_239_421,
        source_type: SourceType::Wikidata,
        name: "HMAPPS Document",
        extensions: &["mv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
