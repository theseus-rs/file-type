use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_733753614: FileFormat = FileFormat {
    id: 733_753_614,
    source_type: SourceType::Iana,
    name: "vnd.trid.tpt",
    extensions: &[],
    media_types: &["application/vnd.trid.tpt"],
    internal_signatures: &[],
    related_formats: &[],
};
