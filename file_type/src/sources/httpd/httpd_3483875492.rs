use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3483875492: FileFormat = FileFormat {
    id: 3_483_875_492,
    source_type: SourceType::Httpd,
    name: "fujitsu oasys2",
    extensions: &["oa2"],
    media_types: &["application/vnd.fujitsu.oasys2"],
    signatures: &[],
    related_formats: &[],
};
