use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853089: FileFormat = FileFormat {
    id: 105_853_089,
    source_type: SourceType::Wikidata,
    name: "Squeeze Presets",
    extensions: &["spfx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
