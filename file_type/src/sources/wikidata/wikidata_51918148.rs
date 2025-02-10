use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51918148: FileType = FileType {
    file_format: &FileFormat {
        id: 51_918_148,
        source_type: SourceType::Wikidata,
        name: "XYWrite Document",
        extensions: &["xy"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
