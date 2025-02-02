use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12792181662701773000: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "dece zip",
    extensions: &["uvz", "uvvz"],
    media_types: &["application/vnd.dece.zip"],
    internal_signatures: &[],
    related_formats: &[],
};
