use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130642271: FileType = FileType {
    file_format: &FileFormat {
        id: 130_642_271,
        source_type: SourceType::Wikidata,
        name: "Roboconf graph file",
        extensions: &["graph"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
