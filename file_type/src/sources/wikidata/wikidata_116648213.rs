use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116648213: FileType = FileType {
    file_format: &FileFormat {
        id: 116_648_213,
        source_type: SourceType::Wikidata,
        name: "Template file",
        extensions: &["ofl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
