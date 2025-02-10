use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51839192: FileType = FileType {
    file_format: &FileFormat {
        id: 51_839_192,
        source_type: SourceType::Wikidata,
        name: "PHP Script Page",
        extensions: &["php"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
