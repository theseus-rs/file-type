use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34746769: FileType = FileType {
    file_format: &FileFormat {
        id: 34_746_769,
        source_type: SourceType::Wikidata,
        name: "STATISTICA Report File",
        extensions: &["str"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
