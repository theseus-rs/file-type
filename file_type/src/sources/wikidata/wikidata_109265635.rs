use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_109265635: FileType = FileType {
    file_format: &FileFormat {
        id: 109_265_635,
        source_type: SourceType::Wikidata,
        name: "Pro Write Document",
        extensions: &["doc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
