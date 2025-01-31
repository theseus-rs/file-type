use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109239421: FileFormat = FileFormat {
    id: 109_239_421,
    puid: "wikidata/109239421",
    name: "HMAPPS Document",
    extensions: &["mv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
