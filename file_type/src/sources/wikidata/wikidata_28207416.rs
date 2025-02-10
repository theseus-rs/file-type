use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207416: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_416,
        source_type: SourceType::Wikidata,
        name: "VDC BitMap",
        extensions: &["bm", "vbm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
