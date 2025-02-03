use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_597049660: FileFormat = FileFormat {
    id: 597_049_660,
    source_type: SourceType::Httpd,
    name: "srgs xml",
    extensions: &["grxml"],
    media_types: &["application/srgs+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
