use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3075306099: FileFormat = FileFormat {
    id: 3_075_306_099,
    source_type: SourceType::Iana,
    name: "EDI-consent",
    extensions: &[],
    media_types: &["application/EDI-consent"],
    internal_signatures: &[],
    related_formats: &[],
};
