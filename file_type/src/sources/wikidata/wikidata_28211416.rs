use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28211416: FileType = FileType {
    file_format: &FileFormat {
        id: 28_211_416,
        source_type: SourceType::Wikidata,
        name: "Ability Write",
        extensions: &["awp", "aww"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
