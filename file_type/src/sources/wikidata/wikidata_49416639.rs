use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_49416639: FileType = FileType {
    file_format: &FileFormat {
        id: 49_416_639,
        source_type: SourceType::Wikidata,
        name: "CATIA Product Description, version 5",
        extensions: &["catproduct"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
