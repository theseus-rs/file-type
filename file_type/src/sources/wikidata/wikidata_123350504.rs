use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123350504: FileType = FileType {
    file_format: &FileFormat {
        id: 123_350_504,
        source_type: SourceType::Wikidata,
        name: "RootsMagic Chart file",
        extensions: &["rmc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
