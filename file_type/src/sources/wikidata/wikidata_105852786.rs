use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852786: FileFormat = FileFormat {
    id: 105_852_786,
    puid: "wikidata/105852786",
    name: "MoonShell2 skin",
    extensions: &["skn"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2A, 0x53, 0x6B, 0x69, 0x6E, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x73, 0x20, 0x70,
                    0x61, 0x63, 0x6B, 0x61, 0x67, 0x65, 0x20, 0x66, 0x6F, 0x72, 0x20, 0x4D, 0x6F,
                    0x6F, 0x6E, 0x53, 0x68, 0x65, 0x6C, 0x6C, 0x32,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
