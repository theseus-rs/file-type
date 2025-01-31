use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855151: FileFormat = FileFormat {
    id: 105_855_151,
    puid: "wikidata/105855151",
    name: "FBIde session",
    extensions: &["fbs"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x66, 0x62, 0x69, 0x64, 0x65, 0x3A, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6F,
                    0x6E, 0x3A, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x3D, 0x20, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
