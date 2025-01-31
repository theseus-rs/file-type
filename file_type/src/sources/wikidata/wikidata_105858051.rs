use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858051: FileFormat = FileFormat {
    id: 105_858_051,
    puid: "wikidata/105858051",
    name: "InstallSimple Project",
    extensions: &["ispro"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3B, 0x20, 0x49, 0x6E, 0x73, 0x74, 0x61, 0x6C, 0x6C, 0x53, 0x69, 0x6D, 0x70,
                    0x6C, 0x65, 0x20, 0x70, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x0A, 0x0A, 0x5B,
                    0x53, 0x65, 0x74, 0x75, 0x70, 0x5D, 0x0A, 0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77,
                    0x54, 0x69, 0x74, 0x6C, 0x65, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
