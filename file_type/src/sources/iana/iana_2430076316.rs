use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2430076316: FileFormat = FileFormat {
    id: 2_430_076_316,
    source_type: SourceType::Iana,
    name: "vnd.fujitsu.oasys",
    extensions: &[],
    media_types: &["application/vnd.fujitsu.oasys"],
    internal_signatures: &[],
    related_formats: &[],
};
