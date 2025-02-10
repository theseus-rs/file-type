use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116418918: FileType = FileType {
    file_format: &FileFormat {
        id: 116_418_918,
        source_type: SourceType::Wikidata,
        name: "Adobe Photoshop Color Table",
        extensions: &["act"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
