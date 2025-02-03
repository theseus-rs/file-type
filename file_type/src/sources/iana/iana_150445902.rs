use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_150445902: FileFormat = FileFormat {
    id: 150_445_902,
    source_type: SourceType::Iana,
    name: "vnd.onepagertatx",
    extensions: &[],
    media_types: &["application/vnd.onepagertatx"],
    internal_signatures: &[],
    related_formats: &[],
};
