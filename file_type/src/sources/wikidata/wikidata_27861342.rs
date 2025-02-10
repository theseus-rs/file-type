use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27861342: FileType = FileType {
    file_format: &FileFormat {
        id: 27_861_342,
        source_type: SourceType::Wikidata,
        name: "Windows Prefetch File, version 26",
        extensions: &["pf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
