use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_122485070: FileFormat = FileFormat {
    id: 122_485_070,
    source_type: SourceType::Iana,
    name: "scip",
    extensions: &[],
    media_types: &["audio/scip"],
    internal_signatures: &[],
    related_formats: &[],
};
