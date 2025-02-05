use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1560688967: FileFormat = FileFormat {
    id: 1_560_688_967,
    source_type: SourceType::Iana,
    name: "vnd.japannet-verification-wakeup",
    extensions: &[],
    media_types: &["application/vnd.japannet-verification-wakeup"],
    signatures: &[],
    related_formats: &[],
};
