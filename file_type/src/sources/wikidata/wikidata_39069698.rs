use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_39069698: FileType = FileType {
    file_format: &FileFormat {
        id: 39_069_698,
        source_type: SourceType::Wikidata,
        name: "Ion",
        extensions: &["ion"],
        media_types: &["application/ion"],
        signatures: &[],
        related_formats: &[],
    },
};
