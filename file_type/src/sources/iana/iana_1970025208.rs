use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1970025208: FileType = FileType {
    file_format: &FileFormat {
        id: 1_970_025_208,
        source_type: SourceType::Iana,
        name: "vnd.s3sms",
        extensions: &[],
        media_types: &["application/vnd.s3sms"],
        signatures: &[],
        related_formats: &[],
    },
};
