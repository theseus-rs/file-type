use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3065746585: FileFormat = FileFormat {
    id: 3_065_746_585,
    source_type: SourceType::Iana,
    name: "vnd.etsi.asic-e+zip",
    extensions: &[],
    media_types: &["application/vnd.etsi.asic-e+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
