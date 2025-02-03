use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_900299791: FileFormat = FileFormat {
    id: 900_299_791,
    source_type: SourceType::Iana,
    name: "kpml-response+xml",
    extensions: &[],
    media_types: &["application/kpml-response+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
