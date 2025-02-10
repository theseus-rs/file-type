use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130351586: FileType = FileType {
    file_format: &FileFormat {
        id: 130_351_586,
        source_type: SourceType::Wikidata,
        name: "Monkey source code file",
        extensions: &["monkey"],
        media_types: &["text/x-monkey"],
        signatures: &[],
        related_formats: &[],
    },
};
