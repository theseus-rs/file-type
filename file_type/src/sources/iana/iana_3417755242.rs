use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3417755242: FileFormat = FileFormat {
    id: 3_417_755_242,
    source_type: SourceType::Iana,
    name: "vnd.oasis.opendocument.formula",
    extensions: &[],
    media_types: &["application/vnd.oasis.opendocument.formula"],
    internal_signatures: &[],
    related_formats: &[],
};
