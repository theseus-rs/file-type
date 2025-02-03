use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_475080157: FileFormat = FileFormat {
    id: 475_080_157,
    source_type: SourceType::Iana,
    name: "vnd.ecowin.series",
    extensions: &[],
    media_types: &["application/vnd.ecowin.series"],
    internal_signatures: &[],
    related_formats: &[],
};
