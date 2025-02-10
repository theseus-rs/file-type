use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_129486038: FileType = FileType {
    file_format: &FileFormat {
        id: 129_486_038,
        source_type: SourceType::Wikidata,
        name: "DOT language file format",
        extensions: &["dot"],
        media_types: &["text/vnd.graphviz", "text/x-graphviz"],
        signatures: &[],
        related_formats: &[],
    },
};
