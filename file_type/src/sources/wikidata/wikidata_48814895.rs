use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48814895: FileFormat = FileFormat {
    id: 48_814_895,
    puid: "wikidata/48814895",
    name: "ESRI ArcView Project",
    extensions: &["apr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
