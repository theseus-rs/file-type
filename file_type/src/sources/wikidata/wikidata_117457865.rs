use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117457865: FileFormat = FileFormat {
    id: 117_457_865,
    puid: "wikidata/117457865",
    name: "XDOMEA 3.0.0",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
