use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855068: FileFormat = FileFormat {
    id: 105_855_068,
    puid: "wikidata/105855068",
    name: "ABC FlowCharter chart (protected)",
    extensions: &["af3"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x18, 0x00, 0x4A, 0x46, 0x4F, 0x00, 0x73, 0x63, 0x70, 0x70,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
