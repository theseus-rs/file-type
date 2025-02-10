use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967416: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_416,
        source_type: SourceType::Wikidata,
        name: "Voice Sequence",
        extensions: &["vsq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
