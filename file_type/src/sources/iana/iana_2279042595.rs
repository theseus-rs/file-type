use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2279042595: FileType = FileType {
    file_format: &FileFormat {
        id: 2_279_042_595,
        source_type: SourceType::Iana,
        name: "vnd.cxtf",
        extensions: &[],
        media_types: &["application/vnd.cxtf"],
        signatures: &[],
        related_formats: &[],
    },
};
