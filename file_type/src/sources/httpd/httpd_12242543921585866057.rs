use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12242543921585866057: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "fujitsu oasys",
    extensions: &["oas"],
    media_types: &["application/vnd.fujitsu.oasys"],
    internal_signatures: &[],
    related_formats: &[],
};
