use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3421485305706899045: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "nokia n gage symbian install",
    extensions: &["n-gage"],
    media_types: &["application/vnd.nokia.n-gage.symbian.install"],
    internal_signatures: &[],
    related_formats: &[],
};
