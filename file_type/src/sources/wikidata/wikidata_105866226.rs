use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866226: FileFormat = FileFormat {
    id: 105_866_226,
    puid: "wikidata/105866226",
    name: "PC-Axis data (var 2)",
    extensions: &["px"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x58, 0x49, 0x53, 0x2D, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x3D,
                    0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
