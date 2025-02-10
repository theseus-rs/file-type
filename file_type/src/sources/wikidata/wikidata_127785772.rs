use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127785772: FileType = FileType {
    file_format: &FileFormat {
        id: 127_785_772,
        source_type: SourceType::Wikidata,
        name: "MEL script",
        extensions: &["mel"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
