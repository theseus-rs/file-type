use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28446959: FileType = FileType {
    file_format: &FileFormat {
        id: 28_446_959,
        source_type: SourceType::Wikidata,
        name: "Binary Document",
        extensions: &["bdoc"],
        media_types: &["application/vnd.bdoc-1.0"],
        signatures: &[],
        related_formats: &[],
    },
};
