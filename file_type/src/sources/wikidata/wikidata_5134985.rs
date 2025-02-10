use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_5134985: FileType = FileType {
    file_format: &FileFormat {
        id: 5_134_985,
        source_type: SourceType::Wikidata,
        name: "CloneCD Control File",
        extensions: &["ccd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
