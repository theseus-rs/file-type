use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125255959: FileFormat = FileFormat {
    id: 125_255_959,
    puid: "wikidata/125255959",
    name: "Simulation Settings File",
    extensions: &["sim"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
