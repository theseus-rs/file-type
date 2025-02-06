use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27349828: FileFormat = FileFormat {
    id: 27_349_828,
    source_type: SourceType::Wikidata,
    name: "ESRI Arc/Info ASCII Grid",
    extensions: &["asc"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6E, 0x63, 0x6F, 0x6C, 0x73, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
