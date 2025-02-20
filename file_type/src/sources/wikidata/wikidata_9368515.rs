use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_9368515: FileType = FileType {
    file_format: &FileFormat {
        id: 9_368_515,
        source_type: SourceType::Wikidata,
        name: "MFS",
        extensions: &["mfs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
