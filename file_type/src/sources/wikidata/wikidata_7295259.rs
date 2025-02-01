use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7295259: FileFormat = FileFormat {
    id: 7_295_259,
    puid: "wikidata/7295259",
    name: "Raster Document Object",
    extensions: &["rdo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
