use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_617077046: FileFormat = FileFormat {
    id: 617_077_046,
    source_type: SourceType::Httpd,
    name: "ms htmlhelp",
    extensions: &["chm"],
    media_types: &["application/vnd.ms-htmlhelp"],
    internal_signatures: &[],
    related_formats: &[],
};
