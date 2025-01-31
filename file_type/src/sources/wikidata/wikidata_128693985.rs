use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128693985: FileFormat = FileFormat {
    id: 128_693_985,
    puid: "wikidata/128693985",
    name: "Brainfuck file",
    extensions: &["bf"],
    media_types: &["application/x-brainfuck"],
    internal_signatures: &[],
    related_formats: &[],
};
