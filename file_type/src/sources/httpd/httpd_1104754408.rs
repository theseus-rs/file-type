use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1104754408: FileFormat = FileFormat {
    id: 1_104_754_408,
    source_type: SourceType::Httpd,
    name: "xproc xml",
    extensions: &["xpl"],
    media_types: &["application/xproc+xml"],
    signatures: &[],
    related_formats: &[],
};
