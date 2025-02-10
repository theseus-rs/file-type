use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121599337: FileType = FileType {
    file_format: &FileFormat {
        id: 121_599_337,
        source_type: SourceType::Wikidata,
        name: "Hallmark Card Studio Project File",
        extensions: &["hmk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
