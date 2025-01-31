use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857327: FileFormat = FileFormat {
    id: 105_857_327,
    puid: "wikidata/105857327",
    name: "JCreator Project",
    extensions: &["jcp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x21, 0x20, 0x2A, 0x2A, 0x20, 0x4A, 0x43, 0x72, 0x65, 0x61, 0x74, 0x6F, 0x72,
                    0x20, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    0x20, 0x28, 0x43, 0x29, 0x32, 0x30, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
