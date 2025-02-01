use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853072: FileFormat = FileFormat {
    id: 105_853_072,
    puid: "wikidata/105853072",
    name: "Vista camera Script",
    extensions: &["script"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x69, 0x73, 0x74, 0x61, 0x20, 0x53, 0x63, 0x72, 0x69, 0x70, 0x74, 0x20,
                    0x46, 0x69, 0x6C, 0x65, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
