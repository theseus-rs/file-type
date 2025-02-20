use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3306561401: FileType = FileType {
    file_format: &FileFormat {
        id: 3_306_561_401,
        source_type: SourceType::Iana,
        name: "vnd.tao.intent-module-archive",
        extensions: &[],
        media_types: &["application/vnd.tao.intent-module-archive"],
        signatures: &[],
        related_formats: &[],
    },
};
