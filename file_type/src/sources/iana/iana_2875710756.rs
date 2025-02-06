use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2875710756: FileFormat = FileFormat {
    id: 2_875_710_756,
    source_type: SourceType::Iana,
    name: "vnd.ficlab.flb+zip",
    extensions: &[],
    media_types: &["application/vnd.ficlab.flb+zip"],
    signatures: &[],
    related_formats: &[],
};
