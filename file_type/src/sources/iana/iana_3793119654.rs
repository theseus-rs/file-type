use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3793119654: FileType = FileType {
    file_format: &FileFormat {
        id: 3_793_119_654,
        source_type: SourceType::Iana,
        name: "vnd.ms-officetheme",
        extensions: &[],
        media_types: &["application/vnd.ms-officetheme"],
        signatures: &[],
        related_formats: &[],
    },
};
