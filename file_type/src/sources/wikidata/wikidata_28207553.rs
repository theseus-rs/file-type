use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207553: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_553,
        source_type: SourceType::Wikidata,
        name: "XGA",
        extensions: &["xga"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
