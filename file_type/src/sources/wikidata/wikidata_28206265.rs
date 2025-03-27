use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206265: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_265,
        source_type: SourceType::Wikidata,
        name: "HSI Raw",
        extensions: &["hst", "raw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
