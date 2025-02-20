use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
