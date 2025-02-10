use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2167893303: FileType = FileType {
    file_format: &FileFormat {
        id: 2_167_893_303,
        source_type: SourceType::Iana,
        name: "pkix-pkipath",
        extensions: &[],
        media_types: &["application/pkix-pkipath"],
        signatures: &[],
        related_formats: &[],
    },
};
