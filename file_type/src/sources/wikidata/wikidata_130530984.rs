use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130530984: FileFormat = FileFormat {
    id: 130_530_984,
    puid: "wikidata/130530984",
    name: "Promela file format",
    extensions: &["pm", "pml", "pr", "prm", "prom", "promela"],
    media_types: &[
        "text/x-promela",
        "text/x-promela",
        "text/x-promela",
        "text/x-promela",
        "text/x-promela",
        "text/x-promela",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
