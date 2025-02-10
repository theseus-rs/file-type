use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_129177480: FileType = FileType {
    file_format: &FileFormat {
        id: 129_177_480,
        source_type: SourceType::Wikidata,
        name: "Fennel source code file",
        extensions: &["fnl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
