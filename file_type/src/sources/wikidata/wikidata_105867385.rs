use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867385: FileFormat = FileFormat {
    id: 105_867_385,
    source_type: SourceType::Wikidata,
    name: "NorthCAD-3D Drawing (v8)",
    extensions: &["n3d"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x04, 0x00, 0x56, 0x65, 0x72, 0x38, 0x00, 0x80, 0x3B, 0xC5, 0x00, 0x80, 0x3B,
                    0xC5, 0x00, 0x80, 0x3B, 0x45, 0x00, 0x80, 0x3B, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
