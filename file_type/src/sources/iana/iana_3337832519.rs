use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3337832519: FileFormat = FileFormat {
    id: 3_337_832_519,
    source_type: SourceType::Iana,
    name: "vnd.japannet-payment-wakeup",
    extensions: &[],
    media_types: &["application/vnd.japannet-payment-wakeup"],
    internal_signatures: &[],
    related_formats: &[],
};
