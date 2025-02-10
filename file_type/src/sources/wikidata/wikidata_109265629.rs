use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109265629: FileType = FileType {
    file_format: &FileFormat {
        id: 109_265_629,
        source_type: SourceType::Wikidata,
        name: "MultiMate Document",
        extensions: &["doc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
