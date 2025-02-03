use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3834774403: FileFormat = FileFormat {
    id: 3_834_774_403,
    source_type: SourceType::Httpd,
    name: "futuresplash",
    extensions: &["spl"],
    media_types: &["application/x-futuresplash"],
    internal_signatures: &[],
    related_formats: &[],
};
