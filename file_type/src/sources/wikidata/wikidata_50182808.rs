use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50182808: FileType = FileType {
    file_format: &FileFormat {
        id: 50_182_808,
        source_type: SourceType::Wikidata,
        name: "Open Inventor File Format, v2",
        extensions: &["iv"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
