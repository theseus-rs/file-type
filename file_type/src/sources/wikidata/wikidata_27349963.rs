use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27349963: FileType = FileType {
    file_format: &FileFormat {
        id: 27_349_963,
        source_type: SourceType::Wikidata,
        name: "TOPSAR C-Band VV Data",
        extensions: &["vvi2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
