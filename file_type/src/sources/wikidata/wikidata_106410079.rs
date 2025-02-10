use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_106410079: FileType = FileType {
    file_format: &FileFormat {
        id: 106_410_079,
        source_type: SourceType::Wikidata,
        name: "MIRAX/MRXS",
        extensions: &["mrxs"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
