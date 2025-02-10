use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109944419: FileType = FileType {
    file_format: &FileFormat {
        id: 109_944_419,
        source_type: SourceType::Wikidata,
        name: "BriefCase file format",
        extensions: &["brc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
