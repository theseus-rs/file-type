use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132640072: FileType = FileType {
    file_format: &FileFormat {
        id: 132_640_072,
        source_type: SourceType::Wikidata,
        name: "PL/SQL source file",
        extensions: &["pls"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
