use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122730741: FileType = FileType {
    file_format: &FileFormat {
        id: 122_730_741,
        source_type: SourceType::Wikidata,
        name: "Lazer View file",
        extensions: &["lv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
