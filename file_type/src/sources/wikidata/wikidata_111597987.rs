use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111597987: FileFormat = FileFormat {
    id: 111_597_987,
    puid: "wikidata/111597987",
    name: "Painter PIX",
    extensions: &["pix"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x00, 0x01, 0x00]),
                    Token::WildcardCount(266),
                    Token::Literal(&[0x00, 0x11, 0x02, 0xFF, 0x0C, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
