use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125391067: FileFormat = FileFormat {
    id: 125_391_067,
    puid: "wikidata/125391067",
    name: "Neuron MODelling Language file",
    extensions: &["mod"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
