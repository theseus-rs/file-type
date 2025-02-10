use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111190444: FileType = FileType {
    file_format: &FileFormat {
        id: 111_190_444,
        source_type: SourceType::Wikidata,
        name: "Java Script Command File",
        extensions: &["jsf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
