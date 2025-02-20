use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967116: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_116,
        source_type: SourceType::Wikidata,
        name: "ASC Sound Master module",
        extensions: &["asc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
