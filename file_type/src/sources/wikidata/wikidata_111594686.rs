use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111594686: FileType = FileType {
    file_format: &FileFormat {
        id: 111_594_686,
        source_type: SourceType::Wikidata,
        name: "Adobe InDesign Document, version 1",
        extensions: &["ind", "indd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
