use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_528362505: FileFormat = FileFormat {
    id: 528_362_505,
    source_type: SourceType::Iana,
    name: "vnd.evolv.ecig.settings",
    extensions: &[],
    media_types: &["application/vnd.evolv.ecig.settings"],
    internal_signatures: &[],
    related_formats: &[],
};
