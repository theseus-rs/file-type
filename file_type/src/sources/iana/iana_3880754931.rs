use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3880754931: FileType = FileType {
    file_format: &FileFormat {
        id: 3_880_754_931,
        source_type: SourceType::Iana,
        name: "vnd.etsi.cug+xml",
        extensions: &[],
        media_types: &["application/vnd.etsi.cug+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
