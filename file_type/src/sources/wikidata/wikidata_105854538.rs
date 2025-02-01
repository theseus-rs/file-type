use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854538: FileFormat = FileFormat {
    id: 105_854_538,
    puid: "wikidata/105854538",
    name: "Oracle Audio drumkit",
    extensions: &["apk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4F, 0x52, 0x41, 0x43, 0x4C, 0x45, 0x00, 0x01, 0x0C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
