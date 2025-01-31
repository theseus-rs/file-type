use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856825: FileFormat = FileFormat {
    id: 105_856_825,
    puid: "wikidata/105856825",
    name: "GMV input file",
    extensions: &["gmv"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x67, 0x6D, 0x76, 0x69, 0x6E, 0x70, 0x75, 0x74, 0x20, 0x61, 0x73, 0x63, 0x69,
                    0x69, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
