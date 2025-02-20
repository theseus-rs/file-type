use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113495025: FileType = FileType {
    file_format: &FileFormat {
        id: 113_495_025,
        source_type: SourceType::Wikidata,
        name: "Software602 Printer Configuration File",
        extensions: &["cfg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
