use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3651541187: FileType = FileType {
    file_format: &FileFormat {
        id: 3_651_541_187,
        source_type: SourceType::Iana,
        name: "vnd.immervision-ivu",
        extensions: &[],
        media_types: &["application/vnd.immervision-ivu"],
        signatures: &[],
        related_formats: &[],
    },
};
