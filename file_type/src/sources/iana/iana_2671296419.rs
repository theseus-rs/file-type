use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2671296419: FileFormat = FileFormat {
    id: 2_671_296_419,
    source_type: SourceType::Iana,
    name: "jose",
    extensions: &[],
    media_types: &["application/jose"],
    internal_signatures: &[],
    related_formats: &[],
};
