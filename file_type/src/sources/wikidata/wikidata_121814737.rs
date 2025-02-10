use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121814737: FileType = FileType {
    file_format: &FileFormat {
        id: 121_814_737,
        source_type: SourceType::Wikidata,
        name: "Common Interface File",
        extensions: &["cif", "mca"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
