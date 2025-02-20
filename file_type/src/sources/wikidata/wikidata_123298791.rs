use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123298791: FileType = FileType {
    file_format: &FileFormat {
        id: 123_298_791,
        source_type: SourceType::Wikidata,
        name: "Retrospect RXT File",
        extensions: &["rxt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
