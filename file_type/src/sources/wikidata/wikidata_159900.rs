use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_159900: FileType = FileType {
    file_format: &FileFormat {
        id: 159_900,
        source_type: SourceType::Wikidata,
        name: ".htaccess",
        extensions: &["htaccess"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
