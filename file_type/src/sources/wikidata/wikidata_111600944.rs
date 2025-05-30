use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111600944: FileType = FileType {
    file_format: &FileFormat {
        id: 111_600_944,
        source_type: SourceType::Wikidata,
        name: "Adobe InDesign Document, version CC 2014",
        extensions: &["ind", "indd", "indt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
