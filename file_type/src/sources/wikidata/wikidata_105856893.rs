use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856893: FileFormat = FileFormat {
    id: 105_856_893,
    puid: "wikidata/105856893",
    name: "DexDrive memory card save game",
    extensions: &["gme"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x31, 0x32, 0x33, 0x2D, 0x34, 0x35, 0x36, 0x2D, 0x53, 0x54, 0x44, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
