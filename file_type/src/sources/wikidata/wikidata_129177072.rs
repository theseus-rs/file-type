use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_129177072: FileType = FileType {
    file_format: &FileFormat {
        id: 129_177_072,
        source_type: SourceType::Wikidata,
        name: "Fantom source code file",
        extensions: &["fan"],
        media_types: &["application/x-fantom"],
        signatures: &[],
        related_formats: &[],
    },
};
