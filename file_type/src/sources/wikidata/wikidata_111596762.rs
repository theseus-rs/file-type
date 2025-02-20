use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111596762: FileType = FileType {
    file_format: &FileFormat {
        id: 111_596_762,
        source_type: SourceType::Wikidata,
        name: "Adobe InDesign Document, version CC",
        extensions: &["ind", "indd", "indt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
