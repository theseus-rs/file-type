use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_124969775: FileType = FileType {
    file_format: &FileFormat {
        id: 124_969_775,
        source_type: SourceType::Wikidata,
        name: "Songsmith file",
        extensions: &["songsmith"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
