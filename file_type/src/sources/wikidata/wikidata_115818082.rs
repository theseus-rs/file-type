use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_115818082: FileType = FileType {
    file_format: &FileFormat {
        id: 115_818_082,
        source_type: SourceType::Wikidata,
        name: "Interoperable Master Format",
        extensions: &["mxf", "xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
