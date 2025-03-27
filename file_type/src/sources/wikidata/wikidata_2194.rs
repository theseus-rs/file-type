use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2194: FileType = FileType {
    file_format: &FileFormat {
        id: 2_194,
        source_type: SourceType::Wikidata,
        name: "Windows Media Audio",
        extensions: &["wma"],
        media_types: &["audio/x-ms-wma"],
        signatures: &[],
        related_formats: &[],
    },
};
