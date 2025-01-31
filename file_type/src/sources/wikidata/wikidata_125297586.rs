use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125297586: FileFormat = FileFormat {
    id: 125_297_586,
    puid: "wikidata/125297586",
    name: "Scheme program source",
    extensions: &["sps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
