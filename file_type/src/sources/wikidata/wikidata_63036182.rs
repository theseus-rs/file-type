use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_63036182: FileType = FileType {
    file_format: &FileFormat {
        id: 63_036_182,
        source_type: SourceType::Wikidata,
        name: "8-bit ASCII Text",
        extensions: &["asc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
