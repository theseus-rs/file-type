use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3320182908: FileFormat = FileFormat {
    id: 3_320_182_908,
    source_type: SourceType::Iana,
    name: "vnd.oma.cab-subs-invite+xml",
    extensions: &[],
    media_types: &["application/vnd.oma.cab-subs-invite+xml"],
    signatures: &[],
    related_formats: &[],
};
