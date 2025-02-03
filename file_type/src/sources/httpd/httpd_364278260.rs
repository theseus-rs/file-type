use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_364278260: FileFormat = FileFormat {
    id: 364_278_260,
    source_type: SourceType::Httpd,
    name: "xfig",
    extensions: &["fig"],
    media_types: &["application/x-xfig"],
    internal_signatures: &[],
    related_formats: &[],
};
