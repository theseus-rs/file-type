use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112668587: FileFormat = FileFormat {
    id: 112_668_587,
    puid: "wikidata/112668587",
    name: "Lightscape Material",
    extensions: &["atr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
