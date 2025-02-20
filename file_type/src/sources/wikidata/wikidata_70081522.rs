use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_70081522: FileType = FileType {
    file_format: &FileFormat {
        id: 70_081_522,
        source_type: SourceType::Wikidata,
        name: "TextPipe Filter List file format",
        extensions: &["fll"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
