use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_617077046: FileFormat = FileFormat {
    id: 617_077_046,
    source_type: SourceType::Iana,
    name: "vnd.ms-htmlhelp",
    extensions: &[],
    media_types: &["application/vnd.ms-htmlhelp"],
    internal_signatures: &[],
    related_formats: &[],
};
