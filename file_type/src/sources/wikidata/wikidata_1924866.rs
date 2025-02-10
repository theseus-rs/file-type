use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1924866: FileType = FileType {
    file_format: &FileFormat {
        id: 1_924_866,
        source_type: SourceType::Wikidata,
        name: "Metalink",
        extensions: &["meta4", "metalink"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
