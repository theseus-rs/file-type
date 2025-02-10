use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206085: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_085,
        source_type: SourceType::Wikidata,
        name: "TT High Resolution",
        extensions: &["PI7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
