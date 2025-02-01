use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111519484: FileFormat = FileFormat {
    id: 111_519_484,
    puid: "wikidata/111519484",
    name: "ESRI ArcInfo Grid .nit File",
    extensions: &["nit"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
