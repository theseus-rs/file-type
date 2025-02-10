use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112669152: FileType = FileType {
    file_format: &FileFormat {
        id: 112_669_152,
        source_type: SourceType::Wikidata,
        name: "Lightscape Parameter",
        extensions: &["df"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
