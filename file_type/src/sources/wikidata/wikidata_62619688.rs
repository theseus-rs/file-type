use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_62619688: FileType = FileType {
    file_format: &FileFormat {
        id: 62_619_688,
        source_type: SourceType::Wikidata,
        name: "7-bit ASCII Text",
        extensions: &["asc"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
