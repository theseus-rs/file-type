use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118721205: FileFormat = FileFormat {
    id: 118_721_205,
    puid: "wikidata/118721205",
    name: "PZZ File",
    extensions: &["pzz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
