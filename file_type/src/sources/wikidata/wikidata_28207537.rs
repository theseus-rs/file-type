use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207537: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_537,
        source_type: SourceType::Wikidata,
        name: "Xerox Doodle brush",
        extensions: &["brush"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
