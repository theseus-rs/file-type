use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130868730: FileType = FileType {
    file_format: &FileFormat {
        id: 130_868_730,
        source_type: SourceType::Wikidata,
        name: "ShExC file",
        extensions: &["shex"],
        media_types: &["text/shex"],
        signatures: &[],
        related_formats: &[],
    },
};
