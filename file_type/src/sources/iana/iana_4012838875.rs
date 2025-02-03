use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4012838875: FileFormat = FileFormat {
    id: 4_012_838_875,
    source_type: SourceType::Iana,
    name: "vnd.denovo.fcselayout-link",
    extensions: &[],
    media_types: &["application/vnd.denovo.fcselayout-link"],
    internal_signatures: &[],
    related_formats: &[],
};
