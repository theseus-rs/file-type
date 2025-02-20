use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130244670: FileType = FileType {
    file_format: &FileFormat {
        id: 130_244_670,
        source_type: SourceType::Wikidata,
        name: "LiveScript source code file",
        extensions: &["ls"],
        media_types: &["text/livescript"],
        signatures: &[],
        related_formats: &[],
    },
};
