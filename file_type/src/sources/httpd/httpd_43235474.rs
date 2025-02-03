use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_43235474: FileFormat = FileFormat {
    id: 43_235_474,
    source_type: SourceType::Httpd,
    name: "oasis opendocument graphics template",
    extensions: &["otg"],
    media_types: &["application/vnd.oasis.opendocument.graphics-template"],
    internal_signatures: &[],
    related_formats: &[],
};
