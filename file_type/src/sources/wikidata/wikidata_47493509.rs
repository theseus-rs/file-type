use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47493509: FileType = FileType {
    file_format: &FileFormat {
        id: 47_493_509,
        source_type: SourceType::Wikidata,
        name: "Adobe InDesign Document, version CS2",
        extensions: &["ind", "indd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
