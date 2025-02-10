use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116646759: FileType = FileType {
    file_format: &FileFormat {
        id: 116_646_759,
        source_type: SourceType::Wikidata,
        name: "eXcelon Studio project",
        extensions: &["prj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
