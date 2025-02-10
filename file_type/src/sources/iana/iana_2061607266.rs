use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2061607266: FileType = FileType {
    file_format: &FileFormat {
        id: 2_061_607_266,
        source_type: SourceType::Iana,
        name: "geopose+json",
        extensions: &[],
        media_types: &["application/geopose+json"],
        signatures: &[],
        related_formats: &[],
    },
};
