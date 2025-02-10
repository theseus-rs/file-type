use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_26697935: FileType = FileType {
    file_format: &FileFormat {
        id: 26_697_935,
        source_type: SourceType::Wikidata,
        name: "PHP script",
        extensions: &["php"],
        media_types: &["text/x-php"],
        signatures: &[],
        related_formats: &[],
    },
};
