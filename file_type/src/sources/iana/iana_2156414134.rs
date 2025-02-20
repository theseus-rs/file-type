use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2156414134: FileType = FileType {
    file_format: &FileFormat {
        id: 2_156_414_134,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.ussd+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.ussd+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
