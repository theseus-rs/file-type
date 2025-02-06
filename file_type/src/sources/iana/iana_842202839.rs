use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_842202839: FileFormat = FileFormat {
    id: 842_202_839,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.spreadsheetml.table+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml"],
    signatures: &[],
    related_formats: &[],
};
