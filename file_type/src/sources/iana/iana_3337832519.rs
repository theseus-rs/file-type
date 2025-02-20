use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3337832519: FileType = FileType {
    file_format: &FileFormat {
        id: 3_337_832_519,
        source_type: SourceType::Iana,
        name: "vnd.japannet-payment-wakeup",
        extensions: &[],
        media_types: &["application/vnd.japannet-payment-wakeup"],
        signatures: &[],
        related_formats: &[],
    },
};
