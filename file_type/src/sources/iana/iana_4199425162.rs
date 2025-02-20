use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4199425162: FileType = FileType {
    file_format: &FileFormat {
        id: 4_199_425_162,
        source_type: SourceType::Iana,
        name: "vnd.ms-office.activeX+xml",
        extensions: &[],
        media_types: &["application/vnd.ms-office.activeX+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
