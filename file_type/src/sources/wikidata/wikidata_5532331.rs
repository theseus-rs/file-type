use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5532331: FileFormat = FileFormat {
    id: 5_532_331,
    puid: "wikidata/5532331",
    name: "General content descriptor",
    extensions: &["gcd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x6F, 0x6E, 0x74, 0x65, 0x6E, 0x74, 0x2D, 0x54, 0x79, 0x70, 0x65, 0x3A,
                    0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
