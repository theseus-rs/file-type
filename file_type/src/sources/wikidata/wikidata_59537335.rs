use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_59537335: FileType = FileType {
    file_format: &FileFormat {
        id: 59_537_335,
        source_type: SourceType::Wikidata,
        name: "Apple iWorks Keynote",
        extensions: &["key"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
