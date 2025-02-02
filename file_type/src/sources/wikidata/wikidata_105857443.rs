use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857443: FileFormat = FileFormat {
    id: 105_857_443,
    source_type: SourceType::Wikidata,
    name: "3D Construction Kit 2 Area",
    extensions: &["3ad"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x6E, 0x20, 0x41, 0x52, 0x45, 0x41, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20,
                    0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20,
                    0x33, 0x64, 0x20, 0x43, 0x6F, 0x6E, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x69,
                    0x6F, 0x6E, 0x20, 0x4B, 0x69, 0x74, 0x20, 0x32, 0x0A, 0x0D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
