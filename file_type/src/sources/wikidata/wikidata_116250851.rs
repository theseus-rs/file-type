use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116250851: FileType = FileType {
    file_format: &FileFormat {
        id: 116_250_851,
        source_type: SourceType::Wikidata,
        name: "CodeWarrior Project file",
        extensions: &["mcp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
