use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859642: FileFormat = FileFormat {
    id: 105_859_642,
    puid: "wikidata/105859642",
    name: "Virtual CD v4 and older",
    extensions: &["vc4"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x56, 0x43, 0x44, 0x43, 0x6F, 0x6E, 0x74, 0x61, 0x69, 0x6E, 0x65, 0x72,
                    0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
