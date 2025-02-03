use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_381753813: FileFormat = FileFormat {
    id: 381_753_813,
    source_type: SourceType::Iana,
    name: "mpeg4-generic",
    extensions: &[],
    media_types: &["application/mpeg4-generic"],
    internal_signatures: &[],
    related_formats: &[],
};
