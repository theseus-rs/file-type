use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_21834748: FileType = FileType {
    file_format: &FileFormat {
        id: 21_834_748,
        source_type: SourceType::Wikidata,
        name: "Adobe Color Swatch",
        extensions: &["aco"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
