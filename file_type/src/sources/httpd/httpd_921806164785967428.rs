use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_921806164785967428: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "hbci",
    extensions: &["hbci"],
    media_types: &["application/vnd.hbci"],
    internal_signatures: &[],
    related_formats: &[],
};
