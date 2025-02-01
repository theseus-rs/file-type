use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855644: FileFormat = FileFormat {
    id: 105_855_644,
    puid: "wikidata/105855644",
    name: "Office Upgrade Control",
    extensions: &["opc"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x23, 0x23, 0x23, 0x20, 0x4F, 0x50, 0x43, 0x20, 0x46, 0x49, 0x4C, 0x45,
                    0x20, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
