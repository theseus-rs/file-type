use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
