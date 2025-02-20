use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600772: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_772,
        source_type: SourceType::Wikidata,
        name: "EnCase hash map",
        extensions: &["EnMap"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
