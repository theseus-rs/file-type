use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2790389749: FileType = FileType {
    file_format: &FileFormat {
        id: 2_790_389_749,
        source_type: SourceType::Iana,
        name: "private-token-issuer-directory",
        extensions: &[],
        media_types: &["application/private-token-issuer-directory"],
        signatures: &[],
        related_formats: &[],
    },
};
