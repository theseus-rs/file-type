use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27966884: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_884,
        source_type: SourceType::Wikidata,
        name: "Direct Stream Digital Audio",
        extensions: &["dsf", "dsflib", "minidsf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
