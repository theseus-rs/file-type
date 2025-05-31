use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105856746: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_746,
        source_type: SourceType::Wikidata,
        name: "Unreal Engine Plugin",
        extensions: &["uplugin"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
