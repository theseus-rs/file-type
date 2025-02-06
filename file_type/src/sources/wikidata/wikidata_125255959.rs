use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125255959: FileFormat = FileFormat {
    id: 125_255_959,
    source_type: SourceType::Wikidata,
    name: "Simulation Settings File",
    extensions: &["sim"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
