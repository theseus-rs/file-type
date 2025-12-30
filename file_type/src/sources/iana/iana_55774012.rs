use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_55774012: FileType = FileType {
    file_format: &FileFormat {
        id: 55_774_012,
        source_type: SourceType::Iana,
        name: "vec-package+zip",
        extensions: &[],
        media_types: &["application/vec-package+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
