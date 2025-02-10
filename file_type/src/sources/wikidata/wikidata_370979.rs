use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_370979: FileType = FileType {
    file_format: &FileFormat {
        id: 370_979,
        source_type: SourceType::Wikidata,
        name: "Amigaguide",
        extensions: &["guide"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
