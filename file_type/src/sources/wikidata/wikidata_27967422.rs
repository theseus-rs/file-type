use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967422: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_422,
        source_type: SourceType::Wikidata,
        name: "ChordML",
        extensions: &["cml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
