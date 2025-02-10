use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_62619668: FileType = FileType {
    file_format: &FileFormat {
        id: 62_619_668,
        source_type: SourceType::Wikidata,
        name: "7-bit ANSI Text",
        extensions: &["ans"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
