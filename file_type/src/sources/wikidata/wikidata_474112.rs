use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_474112: FileType = FileType {
    file_format: &FileFormat {
        id: 474_112,
        source_type: SourceType::Wikidata,
        name: "JHTML",
        extensions: &["jhtml"],
        media_types: &["java-internal/java-html"],
        signatures: &[],
        related_formats: &[],
    },
};
