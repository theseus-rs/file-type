use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127700023: FileType = FileType {
    file_format: &FileFormat {
        id: 127_700_023,
        source_type: SourceType::Wikidata,
        name: "Gravity file",
        extensions: &["grv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
