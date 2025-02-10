use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51718267: FileType = FileType {
    file_format: &FileFormat {
        id: 51_718_267,
        source_type: SourceType::Wikidata,
        name: "Schedule+ Contacts",
        extensions: &["scd"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
