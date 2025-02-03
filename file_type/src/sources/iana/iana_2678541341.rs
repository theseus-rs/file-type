use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2678541341: FileFormat = FileFormat {
    id: 2_678_541_341,
    source_type: SourceType::Iana,
    name: "vnd.tml",
    extensions: &[],
    media_types: &["application/vnd.tml"],
    internal_signatures: &[],
    related_formats: &[],
};
