use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856015: FileFormat = FileFormat {
    id: 105_856_015,
    puid: "wikidata/105856015",
    name: "Ease Database Definition",
    extensions: &["def"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4F, 0x50, 0x45, 0x52, 0x41, 0x1A, 0x00, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
