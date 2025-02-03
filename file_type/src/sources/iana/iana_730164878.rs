use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_730164878: FileFormat = FileFormat {
    id: 730_164_878,
    source_type: SourceType::Iana,
    name: "vnd.amazon.mobi8-ebook",
    extensions: &[],
    media_types: &["application/vnd.amazon.mobi8-ebook"],
    internal_signatures: &[],
    related_formats: &[],
};
