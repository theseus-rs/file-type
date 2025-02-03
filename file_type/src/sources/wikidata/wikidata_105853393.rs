use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853393: FileFormat = FileFormat {
    id: 105_853_393,
    source_type: SourceType::Wikidata,
    name: "Panda Signature file system",
    extensions: &["sig"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x69, 0x67, 0x6E, 0x61, 0x74, 0x75, 0x72, 0x65, 0x20, 0x66, 0x69, 0x6C,
                    0x65, 0x20, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6D, 0x20, 0x28, 0x63, 0x29, 0x20,
                    0x50, 0x61, 0x6E, 0x64, 0x61, 0x20, 0x53, 0x6F, 0x66, 0x74, 0x77, 0x61, 0x72,
                    0x65, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
