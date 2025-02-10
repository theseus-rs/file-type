use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130472499: FileType = FileType {
    file_format: &FileFormat {
        id: 130_472_499,
        source_type: SourceType::Wikidata,
        name: "Pig source code file",
        extensions: &["pig"],
        media_types: &["text/x-pig"],
        signatures: &[],
        related_formats: &[],
    },
};
