use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_58800154: FileType = FileType {
    file_format: &FileFormat {
        id: 58_800_154,
        source_type: SourceType::Wikidata,
        name: "PowerProject",
        extensions: &["pp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
