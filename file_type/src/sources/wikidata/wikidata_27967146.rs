use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967146: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_146,
        source_type: SourceType::Wikidata,
        name: "Eureka Packer module",
        extensions: &["eu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
