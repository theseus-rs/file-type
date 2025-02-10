use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_920489276: FileType = FileType {
    file_format: &FileFormat {
        id: 920_489_276,
        source_type: SourceType::Iana,
        name: "geopackage+sqlite3",
        extensions: &[],
        media_types: &["application/geopackage+sqlite3"],
        signatures: &[],
        related_formats: &[],
    },
};
