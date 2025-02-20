use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111601199: FileType = FileType {
    file_format: &FileFormat {
        id: 111_601_199,
        source_type: SourceType::Wikidata,
        name: "Adobe InDesign Document, version CC 2017",
        extensions: &["indd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
