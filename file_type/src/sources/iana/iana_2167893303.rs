use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2167893303: FileFormat = FileFormat {
    id: 2_167_893_303,
    source_type: SourceType::Iana,
    name: "pkix-pkipath",
    extensions: &[],
    media_types: &["application/pkix-pkipath"],
    internal_signatures: &[],
    related_formats: &[],
};
