use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600470: FileFormat = FileFormat {
    id: 28_600_470,
    puid: "wikidata/28600470",
    name: "DER encoded RSA private key",
    extensions: &["key"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
