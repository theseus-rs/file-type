use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_686403110: FileType = FileType {
    file_format: &FileFormat {
        id: 686_403_110,
        source_type: SourceType::Iana,
        name: "sofa",
        extensions: &[],
        media_types: &["audio/sofa"],
        signatures: &[],
        related_formats: &[],
    },
};
