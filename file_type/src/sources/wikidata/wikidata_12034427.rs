use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_12034427: FileType = FileType {
    file_format: &FileFormat {
        id: 12_034_427,
        source_type: SourceType::Wikidata,
        name: "LuraDocument Format",
        extensions: &["ldf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
