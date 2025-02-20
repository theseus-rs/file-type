use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131232260: FileType = FileType {
    file_format: &FileFormat {
        id: 131_232_260,
        source_type: SourceType::Wikidata,
        name: "Allotrope Simple Model",
        extensions: &["json"],
        media_types: &["application/json"],
        signatures: &[],
        related_formats: &[],
    },
};
