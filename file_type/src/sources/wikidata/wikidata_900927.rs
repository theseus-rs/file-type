use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_900927: FileType = FileType {
    file_format: &FileFormat {
        id: 900_927,
        source_type: SourceType::Wikidata,
        name: "DOT language",
        extensions: &["dot", "gv"],
        media_types: &["text/vnd.graphviz"],
        signatures: &[],
        related_formats: &[],
    },
};
