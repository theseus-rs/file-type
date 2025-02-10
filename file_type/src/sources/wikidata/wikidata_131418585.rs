use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131418585: FileType = FileType {
    file_format: &FileFormat {
        id: 131_418_585,
        source_type: SourceType::Wikidata,
        name: "wdiff file format",
        extensions: &["wdiff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
