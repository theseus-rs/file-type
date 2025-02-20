use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_121599354: FileType = FileType {
    file_format: &FileFormat {
        id: 121_599_354,
        source_type: SourceType::Wikidata,
        name: "Hallmark Card Studio format",
        extensions: &["hcs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
