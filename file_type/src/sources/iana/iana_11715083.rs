use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_11715083: FileType = FileType {
    file_format: &FileFormat {
        id: 11_715_083,
        source_type: SourceType::Iana,
        name: "vnd.visio",
        extensions: &[],
        media_types: &["application/vnd.visio"],
        signatures: &[],
        related_formats: &[],
    },
};
