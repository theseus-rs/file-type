use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_85708012: FileType = FileType {
    file_format: &FileFormat {
        id: 85_708_012,
        source_type: SourceType::Wikidata,
        name: "Calendar Creator Event 3-4",
        extensions: &["ce3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
