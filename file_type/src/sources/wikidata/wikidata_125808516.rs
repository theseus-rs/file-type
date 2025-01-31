use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125808516: FileFormat = FileFormat {
    id: 125_808_516,
    puid: "wikidata/125808516",
    name: "Mnemosyne Flash-card Collection",
    extensions: &["mem"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
