use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10930448860814577903: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "fujitsu oasys3",
    extensions: &["oa3"],
    media_types: &["application/vnd.fujitsu.oasys3"],
    internal_signatures: &[],
    related_formats: &[],
};
