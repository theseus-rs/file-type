use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854108: FileFormat = FileFormat {
    id: 105_854_108,
    puid: "wikidata/105854108",
    name: "Alpha Four Script",
    extensions: &["scp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xBB, 0x00, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
