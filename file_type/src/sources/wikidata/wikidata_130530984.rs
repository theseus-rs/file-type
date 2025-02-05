use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130530984: FileFormat = FileFormat {
    id: 130_530_984,
    source_type: SourceType::Wikidata,
    name: "Promela file format",
    extensions: &["pm", "pml", "pr", "prm", "prom", "promela"],
    media_types: &["text/x-promela"],
    signatures: &[],
    related_formats: &[],
};
