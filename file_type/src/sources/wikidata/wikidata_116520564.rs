use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116520564: FileType = FileType {
    file_format: &FileFormat {
        id: 116_520_564,
        source_type: SourceType::Wikidata,
        name: "PHP 3 Web Page",
        extensions: &["php3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
