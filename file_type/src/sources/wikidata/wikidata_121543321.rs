use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121543321: FileFormat = FileFormat {
    id: 121_543_321,
    puid: "wikidata/121543321",
    name: "TaxCut 2008 Tax Return File",
    extensions: &["t08"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
