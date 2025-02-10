use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111263298: FileType = FileType {
    file_format: &FileFormat {
        id: 111_263_298,
        source_type: SourceType::Wikidata,
        name: "Digilink format",
        extensions: &["dig"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
