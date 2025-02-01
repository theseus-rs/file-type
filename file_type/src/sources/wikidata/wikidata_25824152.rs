use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_25824152: FileFormat = FileFormat {
    id: 25_824_152,
    puid: "wikidata/25824152",
    name: "JOSM Session File",
    extensions: &["jos"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
