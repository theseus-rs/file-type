use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3086943397: FileFormat = FileFormat {
    id: 3_086_943_397,
    source_type: SourceType::Httpd,
    name: "oasis opendocument spreadsheet template",
    extensions: &["ots"],
    media_types: &["application/vnd.oasis.opendocument.spreadsheet-template"],
    internal_signatures: &[],
    related_formats: &[],
};
