use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_120077190: FileType = FileType {
    file_format: &FileFormat {
        id: 120_077_190,
        source_type: SourceType::Wikidata,
        name: "ElectricImage IMAGE",
        extensions: &["ei", "img"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
