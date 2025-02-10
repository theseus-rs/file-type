use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125391067: FileType = FileType {
    file_format: &FileFormat {
        id: 125_391_067,
        source_type: SourceType::Wikidata,
        name: "Neuron MODelling Language file",
        extensions: &["mod"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
