use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2175160977: FileFormat = FileFormat {
    id: 2_175_160_977,
    source_type: SourceType::Iana,
    name: "ATF",
    extensions: &[],
    media_types: &["application/ATF"],
    signatures: &[],
    related_formats: &[],
};
