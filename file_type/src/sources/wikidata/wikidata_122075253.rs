use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122075253: FileType = FileType {
    file_format: &FileFormat {
        id: 122_075_253,
        source_type: SourceType::Wikidata,
        name: "Finale Template File",
        extensions: &["ftm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
