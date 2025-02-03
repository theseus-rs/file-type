use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1224104050: FileFormat = FileFormat {
    id: 1_224_104_050,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.5gsa2x",
    extensions: &[],
    media_types: &["application/vnd.3gpp.5gsa2x"],
    internal_signatures: &[],
    related_formats: &[],
};
