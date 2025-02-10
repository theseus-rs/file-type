use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967124: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_124,
        source_type: SourceType::Wikidata,
        name: "CM3",
        extensions: &["cm3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
