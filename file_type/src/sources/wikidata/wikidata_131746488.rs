use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131746488: FileType = FileType {
    file_format: &FileFormat {
        id: 131_746_488,
        source_type: SourceType::Wikidata,
        name: "Natron Project File",
        extensions: &["ntp"],
        media_types: &["text/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
