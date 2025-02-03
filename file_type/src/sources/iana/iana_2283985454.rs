use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2283985454: FileFormat = FileFormat {
    id: 2_283_985_454,
    source_type: SourceType::Iana,
    name: "nasdata",
    extensions: &[],
    media_types: &["application/nasdata"],
    internal_signatures: &[],
    related_formats: &[],
};
