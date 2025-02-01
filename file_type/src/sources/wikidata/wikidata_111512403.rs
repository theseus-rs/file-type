use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111512403: FileFormat = FileFormat {
    id: 111_512_403,
    puid: "wikidata/111512403",
    name: "ESRI ArcInfo .dat file (external)",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
