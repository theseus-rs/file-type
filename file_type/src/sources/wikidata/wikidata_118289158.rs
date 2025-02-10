use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_118289158: FileType = FileType {
    file_format: &FileFormat {
        id: 118_289_158,
        source_type: SourceType::Wikidata,
        name: "Collection File",
        extensions: &["cfs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
