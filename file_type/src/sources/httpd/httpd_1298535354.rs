use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1298535354: FileFormat = FileFormat {
    id: 1_298_535_354,
    source_type: SourceType::Httpd,
    name: "jcp javame midlet rms",
    extensions: &["rms"],
    media_types: &["application/vnd.jcp.javame.midlet-rms"],
    internal_signatures: &[],
    related_formats: &[],
};
