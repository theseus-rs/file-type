use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47493633: FileType = FileType {
    file_format: &FileFormat {
        id: 47_493_633,
        source_type: SourceType::Wikidata,
        name: "Adobe InDesign Document, version CS6",
        extensions: &["ind", "indd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
