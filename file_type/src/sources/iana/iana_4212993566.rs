use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4212993566: FileFormat = FileFormat {
    id: 4_212_993_566,
    source_type: SourceType::Iana,
    name: "vnd.etsi.tsl.der",
    extensions: &[],
    media_types: &["application/vnd.etsi.tsl.der"],
    internal_signatures: &[],
    related_formats: &[],
};
