use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_139890644: FileType = FileType {
    file_format: &FileFormat {
        id: 139_890_644,
        source_type: SourceType::Wikidata,
        name: "Creo Illustrate Document file",
        extensions: &["c3di"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
