use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3583731876: FileFormat = FileFormat {
    id: 3_583_731_876,
    source_type: SourceType::Httpd,
    name: "nokia n gage symbian install",
    extensions: &["n-gage"],
    media_types: &["application/vnd.nokia.n-gage.symbian.install"],
    signatures: &[],
    related_formats: &[],
};
