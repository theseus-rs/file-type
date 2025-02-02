use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3876443208898332249: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "tex",
    extensions: &["tex"],
    media_types: &["application/x-tex"],
    internal_signatures: &[],
    related_formats: &[],
};
