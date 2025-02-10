use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4028023470: FileType = FileType {
    file_format: &FileFormat {
        id: 4_028_023_470,
        source_type: SourceType::Iana,
        name: "rfc822-headers",
        extensions: &[],
        media_types: &["text/rfc822-headers"],
        signatures: &[],
        related_formats: &[],
    },
};
