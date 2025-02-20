use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130530984: FileType = FileType {
    file_format: &FileFormat {
        id: 130_530_984,
        source_type: SourceType::Wikidata,
        name: "Promela file format",
        extensions: &["pm", "pml", "pr", "prm", "prom", "promela"],
        media_types: &["text/x-promela"],
        signatures: &[],
        related_formats: &[],
    },
};
