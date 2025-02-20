use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_3943569: FileType = FileType {
    file_format: &FileFormat {
        id: 3_943_569,
        source_type: SourceType::Wikidata,
        name: "SEG-Y",
        extensions: &["segy", "sgy"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
