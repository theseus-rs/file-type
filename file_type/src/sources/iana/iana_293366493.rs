use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_293366493: FileFormat = FileFormat {
    id: 293_366_493,
    source_type: SourceType::Iana,
    name: "vnd.etsi.iptvsad-bc+xml",
    extensions: &[],
    media_types: &["application/vnd.etsi.iptvsad-bc+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
