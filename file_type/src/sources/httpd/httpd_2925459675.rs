use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2925459675: FileType = FileType {
    file_format: &FileFormat {
        id: 2_925_459_675,
        source_type: SourceType::Httpd,
        name: "pkix attr cert",
        extensions: &["ac"],
        media_types: &["application/pkix-attr-cert"],
        signatures: &[],
        related_formats: &[],
    },
};
