use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_896475446: FileFormat = FileFormat {
    id: 896_475_446,
    source_type: SourceType::Iana,
    name: "vnd.collabio.xodocuments.presentation",
    extensions: &[],
    media_types: &["application/vnd.collabio.xodocuments.presentation"],
    internal_signatures: &[],
    related_formats: &[],
};
