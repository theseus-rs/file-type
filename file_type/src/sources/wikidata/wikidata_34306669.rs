use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34306669: FileType = FileType {
    file_format: &FileFormat {
        id: 34_306_669,
        source_type: SourceType::Wikidata,
        name: "Scifer archive XML header",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
