use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2925459675: FileType = FileType {
    file_format: &FileFormat {
        id: 2_925_459_675,
        source_type: SourceType::Iana,
        name: "pkix-attr-cert",
        extensions: &[],
        media_types: &["application/pkix-attr-cert"],
        signatures: &[],
        related_formats: &[],
    },
};
