use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855571: FileFormat = FileFormat {
    id: 105_855_571,
    source_type: SourceType::Wikidata,
    name: "C-Worthy Machine Dependant Overlay (v1.x)",
    extensions: &["ovl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x2D, 0x57, 0x6F, 0x72, 0x74, 0x68, 0x79, 0x20, 0x28, 0x52, 0x29, 0x20,
                    0x4D, 0x61, 0x63, 0x68, 0x69, 0x6E, 0x65, 0x20, 0x44, 0x65, 0x70, 0x65, 0x6E,
                    0x64, 0x65, 0x6E, 0x74, 0x20, 0x4F, 0x76, 0x65, 0x72, 0x6C, 0x61, 0x79, 0x20,
                    0x46, 0x69, 0x6C, 0x65, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                    0x31, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
