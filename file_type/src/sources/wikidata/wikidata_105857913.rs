use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857913: FileFormat = FileFormat {
    id: 105_857_913,
    source_type: SourceType::Wikidata,
    name: "Irrlicht 3D scene",
    extensions: &["irr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0xFE, 0x3C, 0x00, 0x3F, 0x00, 0x78, 0x00, 0x6D, 0x00, 0x6C, 0x00, 0x20,
                    0x00, 0x76, 0x00, 0x65, 0x00, 0x72, 0x00, 0x73, 0x00, 0x69, 0x00, 0x6F, 0x00,
                    0x6E, 0x00, 0x3D, 0x00, 0x22, 0x00, 0x31, 0x00, 0x2E, 0x00, 0x30, 0x00, 0x22,
                    0x00, 0x3F, 0x00, 0x3E, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
