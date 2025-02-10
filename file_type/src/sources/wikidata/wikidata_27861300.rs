use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27861300: FileType = FileType {
    file_format: &FileFormat {
        id: 27_861_300,
        source_type: SourceType::Wikidata,
        name: "Windows Prefetch File, version 17",
        extensions: &["pf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
