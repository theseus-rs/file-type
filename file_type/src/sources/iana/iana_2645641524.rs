use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2645641524: FileType = FileType {
    file_format: &FileFormat {
        id: 2_645_641_524,
        source_type: SourceType::Iana,
        name: "mxf",
        extensions: &[],
        media_types: &["application/mxf"],
        signatures: &[],
        related_formats: &[],
    },
};
