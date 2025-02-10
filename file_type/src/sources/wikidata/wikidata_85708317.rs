use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_85708317: FileType = FileType {
    file_format: &FileFormat {
        id: 85_708_317,
        source_type: SourceType::Wikidata,
        name: "Calendar Creator File",
        extensions: &["cc3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
