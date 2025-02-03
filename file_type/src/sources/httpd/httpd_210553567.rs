use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_210553567: FileFormat = FileFormat {
    id: 210_553_567,
    source_type: SourceType::Httpd,
    name: "oasis opendocument presentation template",
    extensions: &["otp"],
    media_types: &["application/vnd.oasis.opendocument.presentation-template"],
    internal_signatures: &[],
    related_formats: &[],
};
