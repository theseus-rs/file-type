use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122069891: FileType = FileType {
    file_format: &FileFormat {
        id: 122_069_891,
        source_type: SourceType::Wikidata,
        name: "Post-It Software Note File",
        extensions: &["ppn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
