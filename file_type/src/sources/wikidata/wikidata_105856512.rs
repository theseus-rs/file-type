use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856512: FileFormat = FileFormat {
    id: 105_856_512,
    puid: "wikidata/105856512",
    name: "PAM Development game data archive",
    extensions: &["wad"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x41, 0x4D, 0x5F, 0x50, 0x41, 0x4B, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
