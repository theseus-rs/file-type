use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206599: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_599,
        source_type: SourceType::Wikidata,
        name: "MIX",
        extensions: &["mix"],
        media_types: &["image/vnd.mix"],
        signatures: &[],
        related_formats: &[],
    },
};
