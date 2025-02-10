use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_403771798: FileType = FileType {
    file_format: &FileFormat {
        id: 403_771_798,
        source_type: SourceType::Iana,
        name: "oblivious-dns-message",
        extensions: &[],
        media_types: &["application/oblivious-dns-message"],
        signatures: &[],
        related_formats: &[],
    },
};
