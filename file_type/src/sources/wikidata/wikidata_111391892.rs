use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111391892: FileType = FileType {
    file_format: &FileFormat {
        id: 111_391_892,
        source_type: SourceType::Wikidata,
        name: "Bryce2 Object",
        extensions: &["obj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
