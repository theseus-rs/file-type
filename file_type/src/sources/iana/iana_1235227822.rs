use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1235227822: FileFormat = FileFormat {
    id: 1_235_227_822,
    source_type: SourceType::Iana,
    name: "vnd.3gpp2.tcap",
    extensions: &[],
    media_types: &["application/vnd.3gpp2.tcap"],
    internal_signatures: &[],
    related_formats: &[],
};
