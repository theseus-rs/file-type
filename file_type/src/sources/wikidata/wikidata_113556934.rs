use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113556934: FileType = FileType {
    file_format: &FileFormat {
        id: 113_556_934,
        source_type: SourceType::Wikidata,
        name: "BlindRead ImageCreator Image",
        extensions: &["iso"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
