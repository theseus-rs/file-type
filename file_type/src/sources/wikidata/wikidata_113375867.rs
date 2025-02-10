use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113375867: FileType = FileType {
    file_format: &FileFormat {
        id: 113_375_867,
        source_type: SourceType::Wikidata,
        name: "Extended GEM Bit Image",
        extensions: &["ximg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
