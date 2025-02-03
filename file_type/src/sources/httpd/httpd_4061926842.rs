use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4061926842: FileFormat = FileFormat {
    id: 4_061_926_842,
    source_type: SourceType::Httpd,
    name: "dece zip",
    extensions: &["uvz", "uvvz"],
    media_types: &["application/vnd.dece.zip"],
    internal_signatures: &[],
    related_formats: &[],
};
