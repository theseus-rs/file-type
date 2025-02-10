use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27861323: FileType = FileType {
    file_format: &FileFormat {
        id: 27_861_323,
        source_type: SourceType::Wikidata,
        name: "Windows Prefetch File, version 23",
        extensions: &["pf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
