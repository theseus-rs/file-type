use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4568607398768371283: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ibm rights management",
    extensions: &["irm"],
    media_types: &["application/vnd.ibm.rights-management"],
    internal_signatures: &[],
    related_formats: &[],
};
