use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206252: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_252,
        source_type: SourceType::Wikidata,
        name: "HMR",
        extensions: &["hmr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
