use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1081730756: FileFormat = FileFormat {
    id: 1_081_730_756,
    source_type: SourceType::Iana,
    name: "vnd.oma.bcast.sgdd+xml",
    extensions: &[],
    media_types: &["application/vnd.oma.bcast.sgdd+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
