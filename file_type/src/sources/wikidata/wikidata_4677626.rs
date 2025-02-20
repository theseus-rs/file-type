use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_4677626: FileType = FileType {
    file_format: &FileFormat {
        id: 4_677_626,
        source_type: SourceType::Wikidata,
        name: "Activity Streams",
        extensions: &["json"],
        media_types: &["application/activity+json"],
        signatures: &[],
        related_formats: &[],
    },
};
