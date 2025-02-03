use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3315716607: FileFormat = FileFormat {
    id: 3_315_716_607,
    source_type: SourceType::Iana,
    name: "vnd.enphase.envoy",
    extensions: &[],
    media_types: &["application/vnd.enphase.envoy"],
    internal_signatures: &[],
    related_formats: &[],
};
