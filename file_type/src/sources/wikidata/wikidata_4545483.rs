use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_4545483: FileType = FileType {
    file_format: &FileFormat {
        id: 4_545_483,
        source_type: SourceType::Wikidata,
        name: "X File Format",
        extensions: &["x"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
