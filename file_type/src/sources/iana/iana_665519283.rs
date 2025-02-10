use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_665519283: FileType = FileType {
    file_format: &FileFormat {
        id: 665_519_283,
        source_type: SourceType::Iana,
        name: "vnd.afpc.foca-codedfont",
        extensions: &[],
        media_types: &["application/vnd.afpc.foca-codedfont"],
        signatures: &[],
        related_formats: &[],
    },
};
