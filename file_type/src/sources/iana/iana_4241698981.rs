use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4241698981: FileFormat = FileFormat {
    id: 4_241_698_981,
    source_type: SourceType::Iana,
    name: "set-registration",
    extensions: &[],
    media_types: &["application/set-registration"],
    internal_signatures: &[],
    related_formats: &[],
};
