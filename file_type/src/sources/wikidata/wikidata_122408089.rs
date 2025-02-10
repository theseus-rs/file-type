use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122408089: FileType = FileType {
    file_format: &FileFormat {
        id: 122_408_089,
        source_type: SourceType::Wikidata,
        name: "PlayStation Debug Executable",
        extensions: &["pse"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
