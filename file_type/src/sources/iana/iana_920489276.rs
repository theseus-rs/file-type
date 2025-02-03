use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_920489276: FileFormat = FileFormat {
    id: 920_489_276,
    source_type: SourceType::Iana,
    name: "geopackage+sqlite3",
    extensions: &[],
    media_types: &["application/geopackage+sqlite3"],
    internal_signatures: &[],
    related_formats: &[],
};
