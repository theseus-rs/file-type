use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_17141186: FileFormat = FileFormat {
    id: 17_141_186,
    puid: "wikidata/17141186",
    name: "Microsoft Help 2",
    extensions: &["hxs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
