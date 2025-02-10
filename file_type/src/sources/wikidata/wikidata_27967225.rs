use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967225: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_225,
        source_type: SourceType::Wikidata,
        name: "D-Lusion Music File",
        extensions: &["dmf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
