use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1813270382: FileType = FileType {
    file_format: &FileFormat {
        id: 1_813_270_382,
        source_type: SourceType::Iana,
        name: "xslt+xml",
        extensions: &[],
        media_types: &["application/xslt+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
