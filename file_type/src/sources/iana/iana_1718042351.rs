use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1718042351: FileFormat = FileFormat {
    id: 1_718_042_351,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.pfcp",
    extensions: &[],
    media_types: &["application/vnd.3gpp.pfcp"],
    internal_signatures: &[],
    related_formats: &[],
};
