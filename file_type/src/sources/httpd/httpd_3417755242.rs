use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3417755242: FileType = FileType {
    file_format: &FileFormat {
        id: 3_417_755_242,
        source_type: SourceType::Httpd,
        name: "oasis opendocument formula",
        extensions: &["odf"],
        media_types: &["application/vnd.oasis.opendocument.formula"],
        signatures: &[],
        related_formats: &[],
    },
};
