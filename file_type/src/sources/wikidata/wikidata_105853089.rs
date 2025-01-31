use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853089: FileFormat = FileFormat {
    id: 105_853_089,
    puid: "wikidata/105853089",
    name: "Squeeze Presets",
    extensions: &["spfx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
