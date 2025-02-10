use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_129177252: FileType = FileType {
    file_format: &FileFormat {
        id: 129_177_252,
        source_type: SourceType::Wikidata,
        name: "Felix source code file",
        extensions: &["flx"],
        media_types: &["text/x-felix"],
        signatures: &[],
        related_formats: &[],
    },
};
