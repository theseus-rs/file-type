use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_233107679: FileFormat = FileFormat {
    id: 233_107_679,
    source_type: SourceType::Iana,
    name: "vnd.avalon+json",
    extensions: &[],
    media_types: &["application/vnd.avalon+json"],
    internal_signatures: &[],
    related_formats: &[],
};
