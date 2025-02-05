use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2125042282: FileFormat = FileFormat {
    id: 2_125_042_282,
    source_type: SourceType::Httpd,
    name: "ibm rights management",
    extensions: &["irm"],
    media_types: &["application/vnd.ibm.rights-management"],
    signatures: &[],
    related_formats: &[],
};
