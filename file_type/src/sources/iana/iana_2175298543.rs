use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2175298543: FileFormat = FileFormat {
    id: 2_175_298_543,
    source_type: SourceType::Iana,
    name: "cybercash",
    extensions: &[],
    media_types: &["application/cybercash"],
    internal_signatures: &[],
    related_formats: &[],
};
