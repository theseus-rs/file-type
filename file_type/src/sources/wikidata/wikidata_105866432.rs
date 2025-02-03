use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866432: FileFormat = FileFormat {
    id: 105_866_432,
    source_type: SourceType::Wikidata,
    name: "PETSCII Editor screen",
    extensions: &["pe"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x7B, 0x22, 0x61, 0x70, 0x70, 0x22, 0x3A, 0x22, 0x50, 0x45, 0x54, 0x53, 0x43,
                    0x49, 0x49, 0x20, 0x45, 0x64, 0x69, 0x74, 0x6F, 0x72, 0x22, 0x2C, 0x22, 0x76,
                    0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x22, 0x3A, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
