use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28447338: FileType = FileType {
    file_format: &FileFormat {
        id: 28_447_338,
        source_type: SourceType::Wikidata,
        name: "Digital Document",
        extensions: &["ddoc"],
        media_types: &["application/ddoc"],
        signatures: &[],
        related_formats: &[],
    },
};
