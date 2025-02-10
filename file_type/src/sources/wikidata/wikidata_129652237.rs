use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_129652237: FileType = FileType {
    file_format: &FileFormat {
        id: 129_652_237,
        source_type: SourceType::Wikidata,
        name: "Igor Pro procedure file",
        extensions: &["ipf"],
        media_types: &["text/ipf"],
        signatures: &[],
        related_formats: &[],
    },
};
