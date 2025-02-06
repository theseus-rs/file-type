use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856384: FileFormat = FileFormat {
    id: 105_856_384,
    source_type: SourceType::Wikidata,
    name: "Dynamo program",
    extensions: &["dyn"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x57, 0x6F, 0x72, 0x6B, 0x73, 0x70, 0x61, 0x63, 0x65, 0x20, 0x56, 0x65,
                    0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
