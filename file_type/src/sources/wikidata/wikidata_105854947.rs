use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854947: FileFormat = FileFormat {
    id: 105_854_947,
    puid: "wikidata/105854947",
    name: "ABC FlowCharter Workspace",
    extensions: &["afw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x18, 0x00, 0x4A, 0x46, 0x4F, 0x00, 0x00, 0x1B, 0xDE, 0x2D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
