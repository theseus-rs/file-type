use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130280361: FileType = FileType {
    file_format: &FileFormat {
        id: 130_280_361,
        source_type: SourceType::Wikidata,
        name: "Mason file format",
        extensions: &["m"],
        media_types: &["application/x-mason"],
        signatures: &[],
        related_formats: &[],
    },
};
