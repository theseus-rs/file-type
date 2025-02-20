use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113495271: FileType = FileType {
    file_format: &FileFormat {
        id: 113_495_271,
        source_type: SourceType::Wikidata,
        name: "Applet Effect Factory Config File",
        extensions: &["data"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
