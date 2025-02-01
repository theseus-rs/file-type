use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27895544: FileFormat = FileFormat {
    id: 27_895_544,
    puid: "wikidata/27895544",
    name: "ADX, version 2",
    extensions: &["adx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
