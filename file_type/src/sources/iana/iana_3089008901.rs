use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3089008901: FileType = FileType {
    file_format: &FileFormat {
        id: 3_089_008_901,
        source_type: SourceType::Iana,
        name: "vnd.d2l.coursepackage1p0+zip",
        extensions: &[],
        media_types: &["application/vnd.d2l.coursepackage1p0+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
