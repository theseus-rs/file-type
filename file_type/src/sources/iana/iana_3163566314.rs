use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3163566314: FileType = FileType {
    file_format: &FileFormat {
        id: 3_163_566_314,
        source_type: SourceType::Iana,
        name: "vnd.dxr",
        extensions: &[],
        media_types: &["application/vnd.dxr"],
        signatures: &[],
        related_formats: &[],
    },
};
