use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_120731961: FileType = FileType {
    file_format: &FileFormat {
        id: 120_731_961,
        source_type: SourceType::Wikidata,
        name: "Amapi Pro file",
        extensions: &["a3p"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
