use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125926204: FileFormat = FileFormat {
    id: 125_926_204,
    puid: "wikidata/125926204",
    name: "Solidworks Slide File",
    extensions: &["sld"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
