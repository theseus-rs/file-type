use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851866: FileFormat = FileFormat {
    id: 105_851_866,
    puid: "wikidata/105851866",
    name: "SPU Playstation log rip",
    extensions: &["spu"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x50, 0x55])],
            },
        }],
    }],
    related_formats: &[],
};
