use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130266674: FileType = FileType {
    file_format: &FileFormat {
        id: 130_266_674,
        source_type: SourceType::Wikidata,
        name: "Luau source code file",
        extensions: &["luau"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
