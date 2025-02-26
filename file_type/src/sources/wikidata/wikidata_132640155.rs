use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132640155: FileType = FileType {
    file_format: &FileFormat {
        id: 132_640_155,
        source_type: SourceType::Wikidata,
        name: "PL/SQL binary",
        extensions: &["plb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
