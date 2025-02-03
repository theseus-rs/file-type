use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2265585215: FileFormat = FileFormat {
    id: 2_265_585_215,
    source_type: SourceType::Iana,
    name: "trig",
    extensions: &[],
    media_types: &["application/trig"],
    internal_signatures: &[],
    related_formats: &[],
};
