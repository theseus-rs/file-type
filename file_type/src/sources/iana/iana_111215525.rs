use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_111215525: FileFormat = FileFormat {
    id: 111_215_525,
    source_type: SourceType::Iana,
    name: "vnd.sealed.xls",
    extensions: &[],
    media_types: &["application/vnd.sealed.xls"],
    internal_signatures: &[],
    related_formats: &[],
};
