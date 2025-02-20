use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_106410286: FileType = FileType {
    file_format: &FileFormat {
        id: 106_410_286,
        source_type: SourceType::Wikidata,
        name: "ZISRAW (CZI)",
        extensions: &["czi"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
