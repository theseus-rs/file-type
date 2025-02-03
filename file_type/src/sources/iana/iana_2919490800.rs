use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2919490800: FileFormat = FileFormat {
    id: 2_919_490_800,
    source_type: SourceType::Iana,
    name: "vnd.mts",
    extensions: &[],
    media_types: &["model/vnd.mts"],
    internal_signatures: &[],
    related_formats: &[],
};
