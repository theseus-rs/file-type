use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1760748: FileType = FileType {
    file_format: &FileFormat {
        id: 1_760_748,
        source_type: SourceType::Wikidata,
        name: "Konqueror website archive",
        extensions: &["war"],
        media_types: &["application/x-webarchive"],
        signatures: &[],
        related_formats: &[],
    },
};
