use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_82521957: FileType = FileType {
    file_format: &FileFormat {
        id: 82_521_957,
        source_type: SourceType::Wikidata,
        name: "Portable Voice format",
        extensions: &["pvf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
