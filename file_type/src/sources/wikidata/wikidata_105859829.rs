use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859829: FileFormat = FileFormat {
    id: 105_859_829,
    puid: "wikidata/105859829",
    name: "Electronic Arts MPC video",
    extensions: &["mpc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x50, 0x43, 0x68])],
            },
        }],
    }],
    related_formats: &[],
};
