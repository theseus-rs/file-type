use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112668587: FileType = FileType {
    file_format: &FileFormat {
        id: 112_668_587,
        source_type: SourceType::Wikidata,
        name: "Lightscape Material",
        extensions: &["atr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
