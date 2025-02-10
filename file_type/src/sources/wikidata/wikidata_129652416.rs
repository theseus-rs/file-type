use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_129652416: FileType = FileType {
    file_format: &FileFormat {
        id: 129_652_416,
        source_type: SourceType::Wikidata,
        name: "Inform 6 template file",
        extensions: &["i6t"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
