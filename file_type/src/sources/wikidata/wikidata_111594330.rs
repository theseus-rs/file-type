use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111594330: FileType = FileType {
    file_format: &FileFormat {
        id: 111_594_330,
        source_type: SourceType::Wikidata,
        name: "Adobe InDesign Library, version 4",
        extensions: &["indl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
