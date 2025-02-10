use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967183: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_183,
        source_type: SourceType::Wikidata,
        name: "FastTracker module",
        extensions: &["ft"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
