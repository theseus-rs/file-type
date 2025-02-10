use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28757999: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_999,
        source_type: SourceType::Wikidata,
        name: "Inform",
        extensions: &["i7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
