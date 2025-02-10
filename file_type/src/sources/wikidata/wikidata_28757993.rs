use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28757993: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_993,
        source_type: SourceType::Wikidata,
        name: "IWA",
        extensions: &["iwa"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
