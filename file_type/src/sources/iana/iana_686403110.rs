use crate::format::{FileFormat, SourceType};
use crate::FileType;

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
