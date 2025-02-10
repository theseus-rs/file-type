use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114891689: FileType = FileType {
    file_format: &FileFormat {
        id: 114_891_689,
        source_type: SourceType::Wikidata,
        name: "Quicken Rental Property Manager File",
        extensions: &["qrp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
