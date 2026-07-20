use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_139890847: FileType = FileType {
    file_format: &FileFormat {
        id: 139_890_847,
        source_type: SourceType::Wikidata,
        name: "PTC Creo Model file",
        extensions: &["ol"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
