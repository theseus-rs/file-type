use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125042416: FileType = FileType {
    file_format: &FileFormat {
        id: 125_042_416,
        source_type: SourceType::Wikidata,
        name: "MIDI-Learn file",
        extensions: &["xly"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
