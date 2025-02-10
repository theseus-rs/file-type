use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3417755242: FileType = FileType {
    file_format: &FileFormat {
        id: 3_417_755_242,
        source_type: SourceType::Iana,
        name: "vnd.oasis.opendocument.formula",
        extensions: &[],
        media_types: &["application/vnd.oasis.opendocument.formula"],
        signatures: &[],
        related_formats: &[],
    },
};
