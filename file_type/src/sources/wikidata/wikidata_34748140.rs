use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34748140: FileType = FileType {
    file_format: &FileFormat {
        id: 34_748_140,
        source_type: SourceType::Wikidata,
        name: "TAP",
        extensions: &["tap"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
