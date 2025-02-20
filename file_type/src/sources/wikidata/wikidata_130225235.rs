use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130225235: FileType = FileType {
    file_format: &FileFormat {
        id: 130_225_235,
        source_type: SourceType::Wikidata,
        name: "Limbo file format",
        extensions: &["b"],
        media_types: &["text/limbo"],
        signatures: &[],
        related_formats: &[],
    },
};
