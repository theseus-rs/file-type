use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1589482: FileType = FileType {
    file_format: &FileFormat {
        id: 1_589_482,
        source_type: SourceType::Wikidata,
        name: "JT",
        extensions: &["JT"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
