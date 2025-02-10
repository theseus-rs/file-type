use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51802416: FileType = FileType {
    file_format: &FileFormat {
        id: 51_802_416,
        source_type: SourceType::Wikidata,
        name: "Calendar Creator Plus Data File",
        extensions: &["cce"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
