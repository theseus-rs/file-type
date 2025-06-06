use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3497436453: FileType = FileType {
    file_format: &FileFormat {
        id: 3_497_436_453,
        source_type: SourceType::Iana,
        name: "vnd.anser-web-certificate-issue-initiation",
        extensions: &[],
        media_types: &["application/vnd.anser-web-certificate-issue-initiation"],
        signatures: &[],
        related_formats: &[],
    },
};
