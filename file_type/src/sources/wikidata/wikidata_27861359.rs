use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27861359: FileType = FileType {
    file_format: &FileFormat {
        id: 27_861_359,
        source_type: SourceType::Wikidata,
        name: "Windows Prefetch File, version 30",
        extensions: &["pf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
