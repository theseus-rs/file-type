use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866780: FileFormat = FileFormat {
    id: 105_866_780,
    source_type: SourceType::Wikidata,
    name: "Image Packaging System Manifest (with transform)",
    extensions: &["p5m"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x74, 0x72, 0x61, 0x6E, 0x73, 0x66, 0x6F, 0x72, 0x6D, 0x20, 0x66, 0x69,
                    0x6C, 0x65, 0x20, 0x70, 0x61, 0x74, 0x68, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
