use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_257854243: FileFormat = FileFormat {
    id: 257_854_243,
    source_type: SourceType::Iana,
    name: "rpki-publication",
    extensions: &[],
    media_types: &["application/rpki-publication"],
    internal_signatures: &[],
    related_formats: &[],
};
