use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855153: FileFormat = FileFormat {
    id: 105_855_153,
    puid: "wikidata/105855153",
    name: "FidoCAD Macro",
    extensions: &["fcm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x4D, 0x41, 0x43, 0x52, 0x4F, 0x43, 0x41, 0x44, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
