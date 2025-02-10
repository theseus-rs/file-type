use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_72274847: FileType = FileType {
    file_format: &FileFormat {
        id: 72_274_847,
        source_type: SourceType::Wikidata,
        name: "Maestro molecular model",
        extensions: &["mae"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
