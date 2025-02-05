use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_597049660: FileFormat = FileFormat {
    id: 597_049_660,
    source_type: SourceType::Iana,
    name: "srgs+xml",
    extensions: &[],
    media_types: &["application/srgs+xml"],
    signatures: &[],
    related_formats: &[],
};
