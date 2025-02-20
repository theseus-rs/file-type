use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3910774729: FileType = FileType {
    file_format: &FileFormat {
        id: 3_910_774_729,
        source_type: SourceType::Iana,
        name: "vnd.tri.onesource",
        extensions: &[],
        media_types: &["application/vnd.tri.onesource"],
        signatures: &[],
        related_formats: &[],
    },
};
