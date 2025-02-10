use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_49416323: FileType = FileType {
    file_format: &FileFormat {
        id: 49_416_323,
        source_type: SourceType::Wikidata,
        name: "CATIA Model (Part Description), version 5",
        extensions: &["catpart"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
