use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856827: FileFormat = FileFormat {
    id: 105_856_827,
    puid: "wikidata/105856827",
    name: "GAPFile (v1.0)",
    extensions: &["gap"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x21, 0x20, 0x47, 0x41, 0x50, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x56, 0x65, 0x72,
                    0x73, 0x69, 0x6F, 0x6E, 0x20, 0x31, 0x2E, 0x30, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
