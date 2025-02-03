use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3725293865: FileFormat = FileFormat {
    id: 3_725_293_865,
    source_type: SourceType::Iana,
    name: "EVRCB1",
    extensions: &[],
    media_types: &["audio/EVRCB1"],
    internal_signatures: &[],
    related_formats: &[],
};
