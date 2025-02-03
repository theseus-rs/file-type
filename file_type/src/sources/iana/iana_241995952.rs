use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_241995952: FileFormat = FileFormat {
    id: 241_995_952,
    source_type: SourceType::Iana,
    name: "vnd.onepagertamp",
    extensions: &[],
    media_types: &["application/vnd.onepagertamp"],
    internal_signatures: &[],
    related_formats: &[],
};
