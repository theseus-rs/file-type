use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_949062720: FileFormat = FileFormat {
    id: 949_062_720,
    source_type: SourceType::Iana,
    name: "vnd.afpc.foca-codepage",
    extensions: &[],
    media_types: &["application/vnd.afpc.foca-codepage"],
    signatures: &[],
    related_formats: &[],
};
