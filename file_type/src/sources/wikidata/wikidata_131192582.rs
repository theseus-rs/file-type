use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131192582: FileFormat = FileFormat {
    id: 131_192_582,
    puid: "wikidata/131192582",
    name: "Uxntal source code file",
    extensions: &["tal"],
    media_types: &["text/x-uxntal"],
    internal_signatures: &[],
    related_formats: &[],
};
