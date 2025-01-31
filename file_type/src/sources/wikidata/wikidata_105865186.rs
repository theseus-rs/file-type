use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865186: FileFormat = FileFormat {
    id: 105_865_186,
    puid: "wikidata/105865186",
    name: "Span-It! Personality definition",
    extensions: &["per"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x54, 0x41, 0x52, 0x54, 0x20, 0x27, 0x53, 0x70, 0x61, 0x6E, 0x69, 0x74,
                    0x27, 0x20, 0x50, 0x65, 0x72, 0x73, 0x6F, 0x6E, 0x61, 0x6C, 0x69, 0x74, 0x79,
                    0x20, 0x44, 0x65, 0x66, 0x69, 0x6E, 0x69, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x2A,
                    0x2A, 0x2A, 0x0D, 0x0A, 0x4E, 0x61, 0x6D, 0x65, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
