use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_96145366: FileFormat = FileFormat {
    id: 96_145_366,
    puid: "wikidata/96145366",
    name: "Wolfram Data Exchange format",
    extensions: &["wdx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
