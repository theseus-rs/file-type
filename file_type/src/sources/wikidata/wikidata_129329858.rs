use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129329858: FileFormat = FileFormat {
    id: 129_329_858,
    puid: "wikidata/129329858",
    name: "Genshi file",
    extensions: &["kid", "kid"],
    media_types: &["application/x-genshi", "application/x-kid"],
    internal_signatures: &[],
    related_formats: &[],
};
