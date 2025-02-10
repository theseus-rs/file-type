use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113577536: FileType = FileType {
    file_format: &FileFormat {
        id: 113_577_536,
        source_type: SourceType::Wikidata,
        name: "WinOnCD Image",
        extensions: &["c2d"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
