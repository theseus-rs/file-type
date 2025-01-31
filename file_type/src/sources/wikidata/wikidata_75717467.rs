use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_75717467: FileFormat = FileFormat {
    id: 75_717_467,
    puid: "wikidata/75717467",
    name: "Visual Studio Project User Options",
    extensions: &["user"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                    0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x20, 0x65, 0x6E, 0x63, 0x6F, 0x64, 0x69,
                    0x6E, 0x67, 0x3D, 0x22, 0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77, 0x73, 0x2D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
