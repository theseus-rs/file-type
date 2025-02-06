use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111597987: FileFormat = FileFormat {
    id: 111_597_987,
    source_type: SourceType::Wikidata,
    name: "Painter PIX",
    extensions: &["pix"],
    media_types: &[],
    signatures: &[Signature {
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
