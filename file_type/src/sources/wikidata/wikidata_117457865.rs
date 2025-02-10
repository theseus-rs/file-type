use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117457865: FileType = FileType {
    file_format: &FileFormat {
        id: 117_457_865,
        source_type: SourceType::Wikidata,
        name: "XDOMEA 3.0.0",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
