use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_668848572: FileFormat = FileFormat {
    id: 668_848_572,
    source_type: SourceType::Httpd,
    name: "fujitsu oasys3",
    extensions: &["oa3"],
    media_types: &["application/vnd.fujitsu.oasys3"],
    internal_signatures: &[],
    related_formats: &[],
};
