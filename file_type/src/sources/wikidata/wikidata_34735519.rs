use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34735519: FileType = FileType {
    file_format: &FileFormat {
        id: 34_735_519,
        source_type: SourceType::Wikidata,
        name: "Signum font",
        extensions: &["e24", "l30", "p24", "p9"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
