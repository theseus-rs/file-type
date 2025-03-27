use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28057842: FileType = FileType {
    file_format: &FileFormat {
        id: 28_057_842,
        source_type: SourceType::Wikidata,
        name: "Axon Raw Format",
        extensions: &["arf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
