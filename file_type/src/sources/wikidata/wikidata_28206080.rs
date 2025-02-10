use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206080: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_080,
        source_type: SourceType::Wikidata,
        name: "PI6",
        extensions: &["PI6"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
