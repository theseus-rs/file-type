use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125255959: FileFormat = FileFormat {
    id: 125_255_959,
    source_type: SourceType::Wikidata,
    name: "Simulation Settings File",
    extensions: &["sim"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
