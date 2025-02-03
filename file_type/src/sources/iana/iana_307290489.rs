use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_307290489: FileFormat = FileFormat {
    id: 307_290_489,
    source_type: SourceType::Iana,
    name: "vnd.cisco.nse",
    extensions: &[],
    media_types: &["audio/vnd.cisco.nse"],
    internal_signatures: &[],
    related_formats: &[],
};
