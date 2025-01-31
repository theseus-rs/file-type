use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855108: FileFormat = FileFormat {
    id: 105_855_108,
    puid: "wikidata/105855108",
    name: "Athena document (v1.x)",
    extensions: &["ath"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x4B, 0x4F, 0x53, 0x5A, 0x54, 0x4F, 0x52, 0x59, 0x53, 0x20, 0x41, 0x54,
                    0x48, 0x45, 0x4E, 0x41, 0x53, 0x4F, 0x46, 0x54, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
