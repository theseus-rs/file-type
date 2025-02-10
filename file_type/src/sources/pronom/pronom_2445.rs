use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2445: FileType = FileType {
    file_format: &FileFormat {
        id: 2_445,
        source_type: SourceType::Pronom,
        name: "SGML/XML Entity File",
        extensions: &["ent"],
        media_types: &["application/xml-external-parsed-entity"],
        signatures: &[],
        related_formats: &[],
    },
};
