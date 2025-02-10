use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_53513821: FileType = FileType {
    file_format: &FileFormat {
        id: 53_513_821,
        source_type: SourceType::Iana,
        name: "cose-x509",
        extensions: &[],
        media_types: &["application/cose-x509"],
        signatures: &[],
        related_formats: &[],
    },
};
