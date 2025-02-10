use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131144297: FileType = FileType {
    file_format: &FileFormat {
        id: 131_144_297,
        source_type: SourceType::Wikidata,
        name: "Solidity source code file",
        extensions: &["sol"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
