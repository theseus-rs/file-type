use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854392: FileFormat = FileFormat {
    id: 105_854_392,
    puid: "wikidata/105854392",
    name: "Dzip compressed archive (v1.x)",
    extensions: &["dz"],
    media_types: &["application/x-dzip"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x5A, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
