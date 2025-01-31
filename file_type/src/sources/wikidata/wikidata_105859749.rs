use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859749: FileFormat = FileFormat {
    id: 105_859_749,
    puid: "wikidata/105859749",
    name: "Psygnosis YOP video",
    extensions: &["yop"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x59, 0x4F, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
