use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2125042282: FileFormat = FileFormat {
    id: 2_125_042_282,
    source_type: SourceType::Iana,
    name: "vnd.ibm.rights-management",
    extensions: &[],
    media_types: &["application/vnd.ibm.rights-management"],
    internal_signatures: &[],
    related_formats: &[],
};
