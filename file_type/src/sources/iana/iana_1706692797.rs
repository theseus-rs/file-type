use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1706692797: FileFormat = FileFormat {
    id: 1_706_692_797,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.sms+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.sms+xml"],
    signatures: &[],
    related_formats: &[],
};
