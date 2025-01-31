use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_777561: FileFormat = FileFormat {
    id: 777_561,
    puid: "wikidata/777561",
    name: "BinHex",
    extensions: &[
        "hcx", "hcx", "hcx", "hex", "hex", "hex", "hqx", "hqx", "hqx",
    ],
    media_types: &[
        "application/binhex",
        "application/binhex",
        "application/binhex",
        "application/mac-binhex",
        "application/mac-binhex",
        "application/mac-binhex",
        "application/mac-binhex40",
        "application/mac-binhex40",
        "application/mac-binhex40",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
