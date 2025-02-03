use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1474921673: FileFormat = FileFormat {
    id: 1_474_921_673,
    source_type: SourceType::Iana,
    name: "vnd.omads-folder+xml",
    extensions: &[],
    media_types: &["application/vnd.omads-folder+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
