use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_132146755: FileType = FileType {
    file_format: &FileFormat {
        id: 132_146_755,
        source_type: SourceType::Wikidata,
        name: "BrailleBlaster ZIP File",
        extensions: &["bbz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
