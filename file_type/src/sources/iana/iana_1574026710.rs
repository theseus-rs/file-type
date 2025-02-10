use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1574026710: FileType = FileType {
    file_format: &FileFormat {
        id: 1_574_026_710,
        source_type: SourceType::Iana,
        name: "ipp",
        extensions: &[],
        media_types: &["application/ipp"],
        signatures: &[],
        related_formats: &[],
    },
};
