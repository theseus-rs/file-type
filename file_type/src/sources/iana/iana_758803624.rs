use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_758803624: FileFormat = FileFormat {
    id: 758_803_624,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
