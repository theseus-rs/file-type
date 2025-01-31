use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128616565: FileFormat = FileFormat {
    id: 128_616_565,
    puid: "wikidata/128616565",
    name: "Asymptote file format",
    extensions: &["asy"],
    media_types: &["text/x-asymptote"],
    internal_signatures: &[],
    related_formats: &[],
};
