use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3653512415: FileType = FileType {
    file_format: &FileFormat {
        id: 3_653_512_415,
        source_type: SourceType::Iana,
        name: "dpx",
        extensions: &[],
        media_types: &["image/dpx"],
        signatures: &[],
        related_formats: &[],
    },
};
