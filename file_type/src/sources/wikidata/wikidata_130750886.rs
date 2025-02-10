use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130750886: FileType = FileType {
    file_format: &FileFormat {
        id: 130_750_886,
        source_type: SourceType::Wikidata,
        name: "Sed script file",
        extensions: &["sed"],
        media_types: &["text/x-sed"],
        signatures: &[],
        related_formats: &[],
    },
};
