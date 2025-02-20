use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_109996883: FileType = FileType {
    file_format: &FileFormat {
        id: 109_996_883,
        source_type: SourceType::Wikidata,
        name: "Primavera P6 Project Management XER File",
        extensions: &["xer"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
