use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28052835: FileType = FileType {
    file_format: &FileFormat {
        id: 28_052_835,
        source_type: SourceType::Wikidata,
        name: "Digital Replica Plus",
        extensions: &["epub"],
        media_types: &["application/epub+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
