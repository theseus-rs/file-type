use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128034125: FileType = FileType {
    file_format: &FileFormat {
        id: 128_034_125,
        source_type: SourceType::Wikidata,
        name: "Rexx source code file",
        extensions: &["arexx", "rex", "rexx", "rx"],
        media_types: &["text/x-rexx"],
        signatures: &[],
        related_formats: &[],
    },
};
