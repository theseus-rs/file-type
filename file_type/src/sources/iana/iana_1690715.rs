use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1690715: FileType = FileType {
    file_format: &FileFormat {
        id: 1_690_715,
        source_type: SourceType::Iana,
        name: "vnd.iccprofile",
        extensions: &[],
        media_types: &["application/vnd.iccprofile"],
        signatures: &[],
        related_formats: &[],
    },
};
