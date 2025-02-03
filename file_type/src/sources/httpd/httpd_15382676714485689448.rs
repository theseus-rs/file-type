use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_15382676714485689448: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "nzb",
    extensions: &["nzb"],
    media_types: &["application/x-nzb"],
    internal_signatures: &[],
    related_formats: &[],
};
