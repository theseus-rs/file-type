use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_320775993: FileFormat = FileFormat {
    id: 320_775_993,
    source_type: SourceType::Iana,
    name: "vnd.dm.delegation+xml",
    extensions: &[],
    media_types: &["application/vnd.dm.delegation+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
