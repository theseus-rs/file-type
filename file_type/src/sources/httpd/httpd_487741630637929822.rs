use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_487741630637929822: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "lzh compressed",
    extensions: &["lzh", "lha"],
    media_types: &["application/x-lzh-compressed"],
    internal_signatures: &[],
    related_formats: &[],
};
