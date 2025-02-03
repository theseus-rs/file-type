use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10445141258930429958: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "fujitsu oasys2",
    extensions: &["oa2"],
    media_types: &["application/vnd.fujitsu.oasys2"],
    internal_signatures: &[],
    related_formats: &[],
};
