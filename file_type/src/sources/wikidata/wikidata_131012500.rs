use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131012500: FileType = FileType {
    file_format: &FileFormat {
        id: 131_012_500,
        source_type: SourceType::Wikidata,
        name: "Stringified NBT format",
        extensions: &["snbt"],
        media_types: &["text/snbt"],
        signatures: &[],
        related_formats: &[],
    },
};
