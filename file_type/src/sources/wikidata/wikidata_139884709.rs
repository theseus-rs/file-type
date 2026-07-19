use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_139884709: FileType = FileType {
    file_format: &FileFormat {
        id: 139_884_709,
        source_type: SourceType::Wikidata,
        name: "Creo View ProductView Structure file",
        extensions: &["pvz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
