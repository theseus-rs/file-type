use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_502801485: FileFormat = FileFormat {
    id: 502_801_485,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
