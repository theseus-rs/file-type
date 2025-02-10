use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128693897: FileType = FileType {
    file_format: &FileFormat {
        id: 128_693_897,
        source_type: SourceType::Wikidata,
        name: "boo script",
        extensions: &["boo"],
        media_types: &["text/x-boo"],
        signatures: &[],
        related_formats: &[],
    },
};
