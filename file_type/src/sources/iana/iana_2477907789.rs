use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2477907789: FileFormat = FileFormat {
    id: 2_477_907_789,
    source_type: SourceType::Iana,
    name: "vnd.lotus-notes",
    extensions: &[],
    media_types: &["application/vnd.lotus-notes"],
    signatures: &[],
    related_formats: &[],
};
