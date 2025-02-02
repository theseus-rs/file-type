use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4956746657128679058: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "osgi subsystem",
    extensions: &["esa"],
    media_types: &["application/vnd.osgi.subsystem"],
    internal_signatures: &[],
    related_formats: &[],
};
