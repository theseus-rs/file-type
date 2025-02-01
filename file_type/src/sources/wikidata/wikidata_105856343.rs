use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856343: FileFormat = FileFormat {
    id: 105_856_343,
    puid: "wikidata/105856343",
    name: "PestPatrol data / scan strings",
    extensions: &["dat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x45, 0x53, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
