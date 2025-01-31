use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856332: FileFormat = FileFormat {
    id: 105_856_332,
    puid: "wikidata/105856332",
    name: "Mozilla Mail folder cache",
    extensions: &["dat"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2F, 0x2F, 0x20, 0x3C, 0x21, 0x2D, 0x2D, 0x20, 0x3C, 0x6D, 0x64, 0x62, 0x3A,
                    0x6D, 0x6F, 0x72, 0x6B, 0x3A, 0x7A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
