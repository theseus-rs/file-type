use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130979811: FileFormat = FileFormat {
    id: 130_979_811,
    puid: "wikidata/130979811",
    name: "Slim file format",
    extensions: &["slim"],
    media_types: &["text/x-slim"],
    internal_signatures: &[],
    related_formats: &[],
};
