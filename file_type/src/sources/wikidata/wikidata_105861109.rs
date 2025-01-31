use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861109: FileFormat = FileFormat {
    id: 105_861_109,
    puid: "wikidata/105861109",
    name: "VirtualBus Language",
    extensions: &["lng"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3B, 0x56, 0x69, 0x72, 0x74, 0x75, 0x61, 0x6C, 0x42, 0x75, 0x73, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
