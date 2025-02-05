use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2213303567: FileFormat = FileFormat {
    id: 2_213_303_567,
    source_type: SourceType::Httpd,
    name: "openxmlformats officedocument spreadsheetml template",
    extensions: &["xltx"],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.template"],
    signatures: &[],
    related_formats: &[],
};
