use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_4972087: FileType = FileType {
    file_format: &FileFormat {
        id: 4_972_087,
        source_type: SourceType::Wikidata,
        name: "Broadcast Markup Language",
        extensions: &[],
        media_types: &["text/X-arib-bml", "text/html"],
        signatures: &[],
        related_formats: &[],
    },
};
