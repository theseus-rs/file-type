use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1239487562: FileFormat = FileFormat {
    id: 1_239_487_562,
    source_type: SourceType::Iana,
    name: "MELP1200",
    extensions: &[],
    media_types: &["audio/MELP1200"],
    internal_signatures: &[],
    related_formats: &[],
};
