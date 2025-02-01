use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861831: FileFormat = FileFormat {
    id: 105_861_831,
    puid: "wikidata/105861831",
    name: "Money Matters data (v4)",
    extensions: &["mm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x4D, 0x44, 0x4A, 0x4D, 0x30, 0x30, 0x56, 0x34,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
