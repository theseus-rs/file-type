use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111530722: FileType = FileType {
    file_format: &FileFormat {
        id: 111_530_722,
        source_type: SourceType::Wikidata,
        name: "SGML/XML Entity File",
        extensions: &["ent"],
        media_types: &["application/xml-external-parsed-entity"],
        signatures: &[],
        related_formats: &[],
    },
};
