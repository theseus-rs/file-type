use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128693985: FileFormat = FileFormat {
    id: 128_693_985,
    source_type: SourceType::Wikidata,
    name: "Brainfuck file",
    extensions: &["bf"],
    media_types: &["application/x-brainfuck"],
    signatures: &[],
    related_formats: &[],
};
