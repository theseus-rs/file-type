use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111392536: FileType = FileType {
    file_format: &FileFormat {
        id: 111_392_536,
        source_type: SourceType::Wikidata,
        name: "Bryce 5 File",
        extensions: &["br5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
