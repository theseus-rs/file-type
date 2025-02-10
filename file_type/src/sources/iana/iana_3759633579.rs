use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3759633579: FileType = FileType {
    file_format: &FileFormat {
        id: 3_759_633_579,
        source_type: SourceType::Iana,
        name: "pkcs8",
        extensions: &[],
        media_types: &["application/pkcs8"],
        signatures: &[],
        related_formats: &[],
    },
};
