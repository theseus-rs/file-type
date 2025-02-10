use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1894544846: FileType = FileType {
    file_format: &FileFormat {
        id: 1_894_544_846,
        source_type: SourceType::Iana,
        name: "vnd.sun.wadl+xml",
        extensions: &[],
        media_types: &["application/vnd.sun.wadl+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
