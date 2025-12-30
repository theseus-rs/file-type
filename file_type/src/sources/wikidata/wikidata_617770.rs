use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_617770: FileType = FileType {
    file_format: &FileFormat {
        id: 617_770,
        source_type: SourceType::Wikidata,
        name: "BBCode",
        extensions: &[],
        media_types: &["text/x-bbcode"],
        signatures: &[],
        related_formats: &[],
    },
};
