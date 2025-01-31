use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854302: FileFormat = FileFormat {
    id: 105_854_302,
    puid: "wikidata/105854302",
    name: "Greenstreet Art drawing (old)",
    extensions: &["art"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x53, 0x54, 0x3A, 0x41, 0x52, 0x54, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
